pub trait GuiBackend {
    type ShaderInfo;
    type Shader;
    type ShaderError;
    type CommandRecorder: CommandRecorder<Backend=Self>;
    type Commands;

    fn create_shader(&self, info: Self::ShaderInfo) -> Result<Self::Shader, Self::ShaderError>;

    fn record_commands(&self) -> Self::CommandRecorder;
}


pub trait CommandRecorder {
    type Backend: GuiBackend;
    
    // fn draw<S>(shader: S, verticies: &[S::Vertex], indicies: Option<&[usize]>) where S: Shader<Self::Backend>;
    
    fn finish(self) -> <Self::Backend as GuiBackend>::Commands;

}