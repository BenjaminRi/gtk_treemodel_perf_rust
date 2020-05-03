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
	U64Column = 0,
}

fn create_store(size: usize) -> TreeStore {
	let treestore = TreeStore::new(&[glib::Type::U32]);
	//Note: If we add a sorted model, the runtime explodes even more
	//let my_store_sort = TreeModelSort::new(&treestore);
	
	{
		let numbers : Vec<u64> = (0..size as u64).rev().collect();
		for number in numbers {
			let iter = treestore.append(None);
			//treestore.set_value(&iter, Columns::U64Column as u32, &number.to_value());
		}
	}
	
	{
		let iter = treestore.get_iter_first().unwrap();
		let new_val = (99 as u64).to_value();
		let time_before = SystemTime::now();
		loop {
			//In this loop, we set `size` u32-elements
			//This operation is somehow taking quadratic amounts of time
			
			/*let _n_u64 = treestore
				.get_value(&iter, Columns::U64Column as i32)
				.get_some::<u64>()
				.unwrap();
			
			//println!("{}", _n_u64);*/
			
			treestore.set_value(&iter,Columns::U64Column as u32, &new_val);
		
			if !treestore.iter_next(&iter) {
				break;
			}
		}
		let elapsed = time_before.elapsed().unwrap();
		println!("{},{}", size, elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64);
	}
	return treestore;
}

fn activate(application: &gtk::Application) {
	let treestore = create_store(15000);
	
	let view = TreeView::new_with_model(&treestore);
	
	let column = TreeViewColumn::new();
	column.set_title("TreeModel Perf");

	let renderer_text = CellRendererText::new();
	column.pack_start(&renderer_text, false);
	column.add_attribute(&renderer_text, "text", Columns::U64Column as i32);
	view.append_column(&column);
	
	let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
	scrolled_window.set_policy(
		gtk::PolicyType::Automatic,
		gtk::PolicyType::Automatic);
	scrolled_window.set_property("overlay-scrolling", &false).unwrap();
	scrolled_window.add(&view);

	let window = ApplicationWindow::new(application);
	window.set_title("TreeModel Perf");
	window.set_default_size(400, 400);
	window.add(&scrolled_window);
	window.show_all();
}

fn main() {
	let application = gtk::Application::new(
		Some("com.github.BenjaminRi.treeview_perf_test"),
		Default::default(),
	)
	.expect("Initialization failed...");

	application.connect_activate(|app| {
		activate(app);
	});

	application.run(&args().collect::<Vec<_>>());
}
