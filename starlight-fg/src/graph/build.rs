use gfx_hal::{buffer, format, image, pass, Backend};
use crate::graph::{
    FrameGraph, PassRef,
    BufferRef, BufferResource, 
    ImageCreateInfo, ImageRef, ImageResource, ImageRead, ImageWrite,
    internal::{FrameGraphInternals, FrameGraphInternalsMut, PassDependency}
};
use crate::pass::{GraphicsPass, ComputePass};

pub trait BuildGraphicsPass<B: Backend> {
    type Output;
    type Pass: GraphicsPass<B>;
    fn build(&self, builder: &mut GraphicsPassBuilder<B>) -> (Self::Output, Self::Pass);
}

impl<T, B: Backend, O, P: GraphicsPass<B>> BuildGraphicsPass<B> for T 
    where T: Fn(&mut GraphicsPassBuilder<B>) -> (O, P) 
{
    type Output = O;
    type Pass = P;
    fn build(&self, builder: &mut GraphicsPassBuilder<B>) -> (O, P) {
        self(builder)
    }
}

pub struct GraphicsPassBuilder<'g, 'p, B: Backend> {
    graph: &'g mut FrameGraph<'p, B>,
    pub(crate) buffers: Vec<BufferResource>,
    pub(crate) images: Vec<ImageResource>,
    pass: PassRef,
}

impl<'g, 'p, B: Backend> GraphicsPassBuilder<'g, 'p, B> {
    pub(crate) fn new(graph: &'g mut FrameGraph<'p, B>, pass: PassRef) -> Self {
        GraphicsPassBuilder { 
            graph: graph,
            buffers: Vec::new(),
            images: Vec::new(),
            pass: pass,
        }
    }

    pub fn create_buffer(&mut self, usage: buffer::Usage) -> BufferRef {
        self.buffers.push(BufferResource {
            usage: usage,
        });
        BufferRef(self.pass, self.buffers.len() - 1)
    }

    pub fn read_buffer(&mut self, buffer: BufferRef) {
        self.graph.graph.add_edge(self.pass.0, (buffer.0).0, PassDependency::ReadBuffer(buffer.1));
    }

    pub fn write_buffer(&mut self, buffer: BufferRef) -> BufferRef {
        let new = self.create_buffer(self.graph.get_buffer(buffer).usage);
        self.graph.graph.add_edge(self.pass.0, (buffer.0).0, PassDependency::CopyBuffer(buffer.1, new.1));
        new
    }

    pub fn create_image(&mut self, info: ImageCreateInfo) -> ImageRef {
        self.images.push(ImageResource {
            info: info,
            write_type: ImageWrite::Transfer,
        });
        ImageRef(self.pass, self.images.len() - 1)
    }

    pub fn read_image(&mut self, image: ImageRef) {
        self.graph.graph.add_edge(
            self.pass.0, (image.0).0, 
            PassDependency::ReadImage(image.1, ImageRead::Transfer)
        );
    }

    pub fn write_image(&mut self, image: ImageRef) -> ImageRef {
        let new = self.create_image(self.graph.get_image(image).info);
        self.graph.graph.add_edge(
            self.pass.0, (image.0).0, 
            PassDependency::CopyImage(image.1, new.1)
        );
        new
    }

    pub fn framebuffer<'b>(&'b mut self) -> FramebufferBuilder<'b, 'g, 'p, B> {
        FramebufferBuilder(self)
    }
    
}

pub struct AttachmentInfo {
    pub format: Option<format::Format>,
    pub samples: image::NumSamples,
}

pub struct FramebufferBuilder<'b, 'g, 'p, B: Backend>(&'b mut GraphicsPassBuilder<'g, 'p, B>);

impl<'b, 'g, 'p, B: Backend> FramebufferBuilder<'b, 'g, 'p, B> {
    pub fn create_attachment(&mut self, image_info: ImageCreateInfo, attachment_info: AttachmentInfo, clear: bool) -> ImageRef {
        let image = self.0.create_image(image_info);
        image
    }

    pub fn read_attachment(&mut self, image: ImageRef, info: AttachmentInfo) {
        self.0.graph.graph.add_edge(
            self.0.pass.0, (image.0).0, 
            PassDependency::ReadImage(image.1, ImageRead::Attachment(info))
        );
    }

    pub fn write_attachment(&mut self, image: ImageRef, info: AttachmentInfo, clear: bool) -> ImageRef {
        let new = self.create_attachment(self.0.graph.get_image(image).info, info, clear);
        self.0.graph.graph.add_edge(
            self.0.pass.0, (image.0).0, 
            PassDependency::CopyImage(image.1, new.1)
        );
        new
    }
}


pub trait BuildComputePass<B: Backend> {
    type Output;
    type Pass: ComputePass<B>;
    fn build(&self, builder: &mut ComputePassBuilder<B>) -> (Self::Output, Self::Pass);
}

impl<T, B: Backend, O, P: ComputePass<B>> BuildComputePass<B> for T 
    where T: Fn(&mut ComputePassBuilder<B>) -> (O, P) 
{
    type Output = O;
    type Pass = P;
    fn build(&self, builder: &mut ComputePassBuilder<B>) -> (O, P) {
        self(builder)
    }
}

pub struct ComputePassBuilder<'g, 'p, B: Backend> {
    graph: &'g mut FrameGraph<'p, B>,
    pub(crate) buffers: Vec<BufferResource>,
    pub(crate) images: Vec<ImageResource>,
    pass: PassRef,
}

impl<'g, 'p, B: Backend> ComputePassBuilder<'g, 'p, B> {
    pub(crate) fn new(graph: &'g mut FrameGraph<'p, B>, pass: PassRef) -> Self {
        ComputePassBuilder { 
            graph: graph,
            buffers: Vec::new(),
            images: Vec::new(),
            pass: pass,
        }
    }

    pub fn create_buffer(&mut self, usage: buffer::Usage) -> BufferRef {
        self.buffers.push(BufferResource {
            usage: usage,
        });
        BufferRef(self.pass, self.buffers.len() - 1)
    }

    pub fn read_buffer(&mut self, buffer: BufferRef) {
        self.graph.graph.add_edge(self.pass.0, (buffer.0).0, PassDependency::ReadBuffer(buffer.1));
    }

    pub fn write_buffer(&mut self, buffer: BufferRef) -> BufferRef {
        let new = self.create_buffer(self.graph.get_buffer(buffer).usage);
        self.graph.graph.add_edge(self.pass.0, (buffer.0).0, PassDependency::CopyBuffer(buffer.1, new.1));
        new
    }

    pub fn create_image(&mut self, info: ImageCreateInfo) -> ImageRef {
        self.images.push(ImageResource {
            info: info,
            write_type: ImageWrite::Transfer,
        });
        ImageRef(self.pass, self.images.len() - 1)
    }

    pub fn read_image(&mut self, image: ImageRef) {
        self.graph.graph.add_edge(
            self.pass.0, (image.0).0, 
            PassDependency::ReadImage(image.1, ImageRead::Transfer)
        );
    }

    pub fn write_image(&mut self, image: ImageRef) -> ImageRef {
        let new = self.create_image(self.graph.get_image(image).info);
        self.graph.graph.add_edge(
            self.pass.0, (image.0).0, 
            PassDependency::CopyImage(image.1, new.1)
        );
        new
    }
}