/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("reviews".to_string(), "New PR reviews"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("open".to_string(), "Open"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "open" => {
                window.show().unwrap();
            }
            "reviews" => {
                todo!()
            }
            _ => {}
        },
        _ => {}
    }
}
