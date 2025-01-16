use crate::*;

#[test]
fn test_login_screen() {
    let harness = Harness::builder()
    .with_size(egui::Vec2::new(300.0, 200.0))
    .build(|ctx| {
        let mut app = OperationApp::new();
        
        
        app.handle_incoming();

        app.handle_action();

        if let Some(id) = app.operation_id {
            app.select_operation(&id);
        }

        if app.staff.is_none() {
            app_component::login(&ctx, &mut app.credential_panel, &mut app.sender, &app.staff);
        } 

        if app.staff.is_some() {
            egui::TopBottomPanel::top("top").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(staff_credential) = app.staff.clone() {
                        ui.horizontal(|ui| {
                            ui.label("name");
                            ui.label(staff_credential.full_name.clone());
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("email");
                            ui.label(staff_credential.email.clone());
                        });
                        if ui.button("logout").clicked() {
                            app.credential_panel = states::Login {
                                field: field::Login {
                                    email: "".to_string(),
                                    password: "".to_string(),
                                    session_token: "".to_string()
                                },
                                state: design::State::Default,
                            };
                            app.search = PreRunning {
                                search_operation: "".to_string(),
                                search_operation_result: vec![]
                            };
                            app.staff = None;
                            app.operation_id = None;
                            app.data = None;
                            app.operation_state = None;
                        }
                    }
                    let current_time = Local::now(); 
                    let formatted_time = format!("Current Time: {}", current_time.format("%Y-%m-%d %H:%M:%S").to_string());
                    let font_id: FontId = FontId::default(); 

                    let text_size = ui.fonts(|font| {
                        let mut calculated_width = 0.0;
                        for char in formatted_time.chars() {
                            calculated_width += font.glyph_width(&font_id, char);
                        }
                        calculated_width
                    });

                    ui.add_space(ui.available_width() - text_size);
                    
                    ui.label(formatted_time);

                });
            });
            egui::SidePanel::left("left").show(ctx, |ui| {
                let margin = 20.0;
                let rect = ui.min_rect();
                let left_panel_rect = rect.center();
                ui.set_max_width(SIDEPANELSIZE);
                
                if let Some(operation_state) = &app.operation_state {
                    if let operation::State::Preoperation(_) = operation_state {
                        if let Some(operation) = app.get_selected_preoperation() {
                            ui.horizontal_wrapped(|ui| {
                                ui.heading("OPERATION: ");       
                                ui.add_enabled(false, 
                                TextEdit::singleline(&mut operation.op_label.to_string())
                                );
                                ui.heading("STATUS: "); 
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut operation.op_status.to_string())
                                );
                                ui.heading("ROOM: ");
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut operation.room_name.to_string())
                                );
                                ui.heading("PATIENT: ");
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut operation.patient_full_name.to_string())
                                );
                                ui.heading("APPROVED CONSENT: ");
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut if operation.approved_consent { "YES".to_string() } else { "NO".to_string() })
                                );
                            });
                        }
                    }
                }
            
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(margin);
                    ui.heading("system by geloquan ");

                    ui.separator();
                });
            });
            egui::SidePanel::right("right").show(ctx, |ui: &mut egui::Ui| {
                ui.set_max_width(SIDEPANELSIZE);
                
                Frame::none()
                .fill(Color32::default())
                .show(ui, |ui| {
                    ui.set_height(ui.available_height());

                    if app.operation_state.is_some() {
                        Frame::none()
                        .inner_margin(Margin::same(20.0))
                        .show(ui, |ui| {
                            ui.heading(RichText::new("Action log"));
                        });
                    };
                    
                    ScrollArea::vertical()
                    .id_salt("action_log")
                    .auto_shrink([false; 2]) 
                    .show(ui, |ui| {
                        if let Some(action_log) = app.get_action_log_operation() {
                            for row in &action_log {
                                ui.label(row.label.clone());
                                ui.horizontal(|ui| {
                                    ui.label("staff: ");
                                    ui.label(row.staff.clone());
                                });
                                ui.horizontal(|ui| {
                                    ui.label("date: ");
                                    ui.label(row.date.clone());
                                });
                                ui.separator();
                            }
                        }
                    });
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if app.staff.is_some() {
                    ui.add_space(80.0);
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        if app.operation_state.is_none() {
                            ui.label("🔎 SEARCH OPERATION");
                            
                            if ui.text_edit_singleline(&mut app.search.search_operation).changed() || 
                            app.require_update == true {
                                let _ = &app.filter_operation();
                                app.require_update = false;
                            }
            
                            ui.separator();
            
                            if app.search.search_operation_result.is_empty() && 
                            app.search.search_operation != "" {
                                ui.label("💤 No results found");
                            } else if app.data.is_some() {
                                if !app.search.search_operation_result.is_empty() {
                                    Frame::none()
                                    .fill(FORM_BACKGROUND)
                                    .rounding(20.0)
                                    .inner_margin(20.0)
                                    .show(ui, |ui| {
                                        ui.columns(1, |columns| {
                                            columns[0].vertical_centered(|ui| {
                                                ui.add_space(20.0);
                                                ui.horizontal_centered(|ui| {
                                                    app.build_table(ui, database::table::window::WindowTable::OperationSelect(Some(app.search.search_operation_result.clone())));
                                                });
                                            });
                                        });
                                    });
                                }
                            }
                        } else if let (
                            Some(operation_state), 
                            Some(_), 
                            Some(data), 
                            app_tx
                        ) = (
                            &mut app.operation_state, 
                            &app.operation_id, 
                            &app.data, 
                            &app.app_tx
                        ) {
                            match operation_state {
                                OperationStates::Preoperation(menu) => {
                                    if let Some(selected_action) = &mut menu.selected_action {
                                        ui.add_space(20.0);
                                        match selected_action {
                                            PreoperativeActions::AddRequirement(s) => {
                                                ui.heading("New Requirement Form");
                                                PreoperativeMenuAction::add_requirement_area(s, data, ui, ctx, app_tx);
                                            },
                                            PreoperativeActions::RemoveRequirement(s) => {
                                                ui.heading("Remove Requirement");
                                                PreoperativeMenuAction::remove_requirement_area(s, data, ui, ctx, app_tx);
                                            },
                                            PreoperativeActions::AddStaffRole => {
                                                
                                            },
                                            PreoperativeActions::RemoveStaffRole => {
                                                
                                            },
                                        }
                                    } else if let Some(selected_menu) = &menu.selected_menu {
                                        match selected_menu {
                                            PreoperativeMenuActionOptions::ToolReady => {
                                                PreoperativeMenuAction::tool_checklist_area(&mut app, ui);
                                            },
                                            PreoperativeMenuActionOptions::Staff => {
                                                PreoperativeMenuAction::staff_list_area(&mut app, ui);
                                            },
                                            PreoperativeMenuActionOptions::EquipmentRequested => {
                                                PreoperativeMenuAction::equipment_requested_area(&mut app, ui);
                                            },
                                        }
                                    
                                    }
                                },
                                OperationStates::Intraoperation(_) => {},
                                OperationStates::Postoperation => todo!(),
                            }           
                        } 
                    });
                } 
            });
            egui::TopBottomPanel::bottom("menu_option").show(ctx, |ui| {
                if let (
                    Some(operation), 
                    Some(operation_state),
                    operation_id, 
                    sender, 
                    staff_credential
                ) = (
                    &mut app.get_selected_preoperation(), 
                    &mut app.operation_state,
                    &app.operation_id,
                    &mut app.sender, 
                    &app.staff
                ) {
                    if let OperationStates::Preoperation(menu) = operation_state {
                        PreoperativeMenu::init(ui, menu, operation, operation_id, sender, staff_credential);
                    }
                } else if let (
                    Some(operation), 
                    Some(operation_state),
                    operation_id, 
                    sender, 
                    staff_credential
                ) = (
                    &mut app.get_selected_intraoperation(), 
                    &mut app.operation_state,
                    &app.operation_id,
                    &mut app.sender, 
                    &app.staff
                ) {
                    if let OperationStates::Intraoperation(menu) = operation_state {
                        IntraoperativeMenu::init(ui, menu, operation, operation_id, sender, staff_credential);
                    }
                }
                ui.add_space(40.0);
            });

            if let Some(operation_state) = &mut app.operation_state {
                egui::TopBottomPanel::bottom("action_option").show(ctx, |ui| {
                    match operation_state {
                        operation::State::Preoperation(menu) => {
                            if let (Some(selected_menu), selected_action) = (menu.selected_menu.as_mut(), &mut menu.selected_action) {
                                match selected_menu {
                                    PreoperativeMenuActionOptions::Staff => {
                                        PreoperativeMenuAction::staff_list_action_options(ui, selected_action);
                                    },
                                    PreoperativeMenuActionOptions::ToolReady => {
                                        PreoperativeMenuAction::tool_ready_action_options(ui, selected_action);
                                    },
                                    PreoperativeMenuActionOptions::EquipmentRequested => {
                                        PreoperativeMenuAction::equipment_requested_options(ui, selected_action);
                                    },
                                }
                                if selected_action.is_none() {
                                } else {
                                    
                                }
                            } else {
                                
                            }
                        
                        },
                        operation::State::Intraoperation(_) => {},
                        operation::State::Postoperation => todo!(),
                    }
                });
            } 
        }

        ctx.request_repaint();
    
    });
    let login_check = harness.query_by_label("login");
    
    assert!(login_check.is_some(), "login check is none!")
}