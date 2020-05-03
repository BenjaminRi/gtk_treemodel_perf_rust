extern crate gio;
extern crate gtk;

use std::env::args;
use std::time::SystemTime;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
	ApplicationWindow, CellRendererText, TreeModelSort, TreeStore, TreeView,
	TreeViewColumn, WindowPosition,
};

enum Columns {
	FirstU32 = 0,
}

fn eval_perf(elems: usize) {
	let my_store = TreeStore::new(&[glib::Type::U32]);
	//let my_store_sort = TreeModelSort::new(&my_store);

	let column = TreeViewColumn::new();
	column.set_title("Column header title");

	let renderer_text = CellRendererText::new();
	column.pack_start(&renderer_text, false);
	column.add_attribute(&renderer_text, "text", Columns::FirstU32 as i32);
	
	let numbers : Vec<u32> = (0..elems as u32).collect();
	for number in numbers {
		my_store.insert_with_values(None, None, &[Columns::FirstU32 as u32], &[&number]);
	}
	
	let iter = my_store.get_iter(&gtk::TreePath::new_first()).unwrap();
	
	let new_val = (99 as u32).to_value();
	let now = SystemTime::now();
	
	//In the following loop, we set 5000 u32-elements
	//This operation takes roughly 240ms on my machine
	loop {
		/*let _n_u32 = my_store
			.get_value(&iter, Columns::FirstU32 as i32)
			.get_some::<u32>()
			.unwrap();
		
		//println!("{}", _n_u32);*/
		
		my_store.set_value(
			&iter,
			Columns::FirstU32 as u32,
			&new_val,
		);
	
		if !my_store.iter_next(&iter) {
			break;
		}
	}
	match now.elapsed() {
		Ok(elapsed) => {
			println!(
				"{},{}",
				elems, elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64
			);
		}
		Err(e) => {
			// an error occurred!
			println!("Error: {:?}", e);
		}
	}
}

fn build_ui(application: &gtk::Application) {
	let window = ApplicationWindow::new(application);

	window.set_title("TreeView performance test");
	window.set_position(WindowPosition::Center);

	let step = 1000;
	let max = 100000;
	for i in (step..=max).step_by(step) {
		eval_perf(i);
	}

	/*let attach_to_view = true;
	if attach_to_view {	
		let my_tree_view = TreeView::new_with_model(&my_store_sort);
		my_tree_view.set_headers_visible(true);
		my_tree_view.append_column(&column);
		let attach_to_window = true;
		if attach_to_window {
			let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
			scrolled_window
				.set_property("overlay-scrolling", &false)
				.unwrap();
			scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
			scrolled_window.add(&my_tree_view);
			window.add(&scrolled_window);
		}
	}*/
	window.show_all();
}

fn main() {
	let application = gtk::Application::new(
		Some("com.github.BenjaminRi.treeview_perf_test"),
		Default::default(),
	)
	.expect("Initialization failed...");

	application.connect_activate(|app| {
		build_ui(app);
	});

	application.run(&args().collect::<Vec<_>>());
}
