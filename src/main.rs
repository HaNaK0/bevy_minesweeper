use bevy::prelude::*;

fn main() {
	App::build()
		.add_system(hello_world_system.system())
		.run();
}

fn hello_world_system() {
	println!("Hello World!")
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(UiCameraBundle::default());

}