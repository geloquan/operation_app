use std::fmt;
use serde::{Serialize, Deserialize};

use super::Tables;


#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum EquipmentStatus {
    Ready,
    Borrowed,
    ForInspection,
}
impl fmt::Display for EquipmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            EquipmentStatus::Ready => "Ready",
            EquipmentStatus::Borrowed => "Borrowed",
            EquipmentStatus::ForInspection => "For Inspection",
        };
        write!(f, "{}", status_str)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum RoomPurpose {
    Patient,
    Surgery,
    Consultation,
    Emergency,
    Storage,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum OperationStatus {
    PreOperative,
    IntraOperative,
    PostOperative,
}
impl fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            OperationStatus::PreOperative => "Pre-Operative",
            OperationStatus::IntraOperative => "Intra-operative",
            OperationStatus::PostOperative => "Post-Operative",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum StaffRole {
    Nurse,
    Technician,
    Pharmacist,
    Dietitian,
    Therapist,
    Physician,
    Maintenance,
    Anesthesiologist,
    SurgicalTechnologist, 
    GeneralSurgeon,
    CardiothoracicSurgeon,
    OrthopedicSurgeon,
    Neurosurgeon,
    Urologist,
    Gynecologist
}
impl fmt::Display for StaffRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            StaffRole::Nurse => "Nurse".to_string(),
            StaffRole::Technician => "Technician".to_string(),
            StaffRole::Pharmacist => "Pharmacist".to_string(),
            StaffRole::Dietitian => "Dietitian".to_string(),
            StaffRole::Therapist => "Therapist".to_string(),
            StaffRole::Physician => "Physician".to_string(),
            StaffRole::Maintenance => "Maintenance".to_string(),
            StaffRole::Anesthesiologist => "Anesthesiologist".to_string(),
            StaffRole::SurgicalTechnologist => "Surgical Technologist".to_string(),
            StaffRole::GeneralSurgeon => "General Surgeon".to_string(),
            StaffRole::CardiothoracicSurgeon => "Cardiothoracic Surgeon".to_string(),
            StaffRole::OrthopedicSurgeon => "Orthopedic Surgeon".to_string(),
            StaffRole::Neurosurgeon => "Neurosurgeon".to_string(),
            StaffRole::Urologist => "Urologist".to_string(),
            StaffRole::Gynecologist => "Gynecologist".to_string(),
        };
        write!(f, "{}", role_str)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum AlertCategory {
    ToolReplacement,
    ToolRequest,
    EquipmentReplacement,
    EquipmentRequest,
    StaffReplacement,
    StaffRequest
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum AlertNotificationStatus {
    Delivered,
    Viewed,
    ActionTaken,
    Resolved
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ActionLogLabel {
    OnSiteToggle,
    AddEquipmentRequirement
}
impl fmt::Display for ActionLogLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action_log_str = match self {
            ActionLogLabel::OnSiteToggle => "Tool On-site toggled",
            ActionLogLabel::AddEquipmentRequirement => "Equipment Requirement Added",
        };
        write!(f, "{}", action_log_str)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ActionLogAction {
    Create,
    Update,
    Delete
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub brand: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Room {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub alias_code: Option<String>,
    pub patient_maximum_occupancy: Option<i32>,
    pub staff_maximum_occupancy: Option<i32>,
    pub purpose: Option<RoomPurpose>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: Option<i32>,
    pub info_id: Option<i32>,
    pub status: Option<EquipmentStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Option<StaffRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolReservation {
    pub id: Option<i32>,
    pub start_time: Option<String>, // Use chrono::NaiveDateTime for datetime
    pub end_time: Option<String>,   // Use chrono::NaiveDateTime for datetime
    pub staff_incharge: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDesignatedRoom {
    pub id: Option<i32>,
    pub room_id: Option<i32>,
    pub tool_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInspector {
    pub id: Option<i32>,
    pub staff_id: Option<i32>,
    pub tool_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: Option<i32>,
    pub label: Option<String>,
    pub status: Option<OperationStatus>,
    pub patient_id: Option<i32>,
    pub room_id: Option<i32>,
    pub start_time: Option<String>, // Use chrono::NaiveDateTime for datetime
    pub end_time: Option<String>,
    pub on_site: Option<i8>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientWardRoom {
    pub id: Option<i32>,
    pub patient_id: Option<i32>,
    pub room_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientWardAssistant {
    pub id: Option<i32>,
    pub staff_id: Option<i32>,
    pub patient_ward_room_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStaff {
    pub id: Option<i32>,
    pub operation_id: Option<i32>,
    pub staff_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationTool {
    pub id: Option<i32>,
    pub operation_id: Option<i32>,
    pub tool_id: Option<i32>,
    pub on_site: Option<i8>, // Use Option<bool> for tinyint (0/1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Option<i32>,
    pub category: Option<AlertCategory>,
    pub operation_id: Option<i32>,
    pub submitted_staff_id: Option<i32>,
    pub notification_status: Option<AlertNotificationStatus>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontdesk {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub address_label: Option<String>,
    pub session_token: Option<String>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertFrontdesk {
    pub id: Option<i32>,
    pub frontdesk_id: Option<i32>,
    pub alert_id: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStaff {
    pub id: Option<i32>,
    pub staff_id: Option<i32>,
    pub alert_id: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLog {
    pub id: Option<i32>,
    pub staff_id: Option<i32>,
    pub label: Option<ActionLogLabel>,
    pub table_name: Option<Tables>,
    pub row_id: Option<i32>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub action: Option<ActionLogAction>,
    pub date_time: Option<String>       
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentRequest {
    pub id: Option<i32>,
    pub operation_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub source_staff_id: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientConsent {
    pub id: Option<i32>,
    pub patient_id: Option<i32>,
    pub approved: Option<i8>
}