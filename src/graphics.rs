//! [wgpu][1] helpers.
//!
//! [1]: https://wgpu.rs

use std::mem::size_of;
use std::path::Path;

use futures_intrusive::channel::shared::oneshot_channel;
use image::{ImageBuffer, Rgba};
use wgpu::{
    BufferAddress,
    BufferDescriptor,
    ImageCopyBuffer,
    ImageCopyTexture,
    ImageDataLayout,
    Instance,
    InstanceDescriptor,
    LoadOp,
    Origin3d,
    PowerPreference,
    RenderPassColorAttachment,
    RequestAdapterOptions,
    TextureAspect,
    TextureDescriptor,
};

use crate::prelude::*;

pub struct Device {
    inner: wgpu::Device,
    queue: wgpu::Queue,
}

pub struct WithTextureView<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    texture_descriptor: TextureDescriptor<'a>,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    width: u32,
    height: u32,
}

pub struct WithBuffer<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    width: u32,
    height: u32,
    texture_descriptor: TextureDescriptor<'a>,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    buffer: wgpu::Buffer,
}

pub struct WithSubmittedCommandBuffer {
    device: wgpu::Device,
    buffer: wgpu::Buffer,
    width: u32,
    height: u32,
}

impl Device {
    #[instrument(skip_all, err)]
    pub async fn new() -> Result<Self> {
        let instance_descriptor = InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        };
        let instance = Instance::new(instance_descriptor);
        let adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        };
        let (inner, queue) = instance
            .request_adapter(&adapter_options)
            .await
            .context("failed to request an adapter")?
            .request_device(&Default::default(), None)
            .await
            .context("failed to request a device")?;
        Ok(Self { inner, queue })
    }

    #[instrument(skip_all, fields(width, height))]
    pub fn create_texture_view<'a>(self, width: u32, height: u32) -> WithTextureView<'a> {
        let descriptor = TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // TODO: 16 bit
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };
        let texture = self.inner.create_texture(&descriptor);
        let view = texture.create_view(&Default::default());
        WithTextureView {
            device: self.inner,
            queue: self.queue,
            texture_descriptor: descriptor,
            texture,
            texture_view: view,
            width,
            height,
        }
    }
}

impl<'a> WithTextureView<'a> {
    pub fn create_output_buffer(self) -> WithBuffer<'a> {
        let size = self.width * self.height * size_of::<u32>() as u32;
        let descriptor = BufferDescriptor {
            size: size as BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let buffer = self.device.create_buffer(&descriptor);
        WithBuffer {
            buffer,
            device: self.device,
            queue: self.queue,
            width: self.width,
            height: self.height,
            texture_descriptor: self.texture_descriptor,
            texture: self.texture,
            texture_view: self.texture_view,
        }
    }
}

impl<'a> WithBuffer<'a> {
    pub fn init_command_encoder(self) -> WithSubmittedCommandBuffer {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let color_attachment = RenderPassColorAttachment {
            view: &self.texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: LoadOp::Clear(wgpu::Color::BLACK),
                store: true,
            },
        };
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
        };
        {
            let render_pass = encoder.begin_render_pass(&render_pass_descriptor);
            // TODO: render_pass.set_pipeline(&render_pipeline);
            // TODO: render_pass.draw(0..3, 0..1);
        }
        encoder.copy_texture_to_buffer(
            ImageCopyTexture {
                aspect: TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            ImageCopyBuffer {
                buffer: &self.buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.width * size_of::<u32>() as u32),
                    rows_per_image: Some(self.height),
                },
            },
            self.texture_descriptor.size,
        );
        self.queue.submit(Some(encoder.finish()));
        WithSubmittedCommandBuffer {
            device: self.device,
            buffer: self.buffer,
            width: self.width,
            height: self.height,
        }
    }
}

impl WithSubmittedCommandBuffer {
    pub async fn render_to(self, path: &Path) -> Result {
        {
            let buffer_slice = self.buffer.slice(..);
            let (tx, rx) = oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.receive()
                .await
                .unwrap()
                .context("failed to map the buffer")?;

            let buffer_view = buffer_slice.get_mapped_range();
            let image_buffer =
                ImageBuffer::<Rgba<u8>, _>::from_raw(self.width, self.height, buffer_view)
                    .expect("container is not big enough");
            image_buffer.save(path)?;
        }
        self.buffer.unmap();
        Ok(())
    }
}
