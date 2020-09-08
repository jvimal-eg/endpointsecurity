// Allow here to prevent compiler errors from the bindgen structs and functions we can't control
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./eps_bindings.rs");

extern crate libc;

use std::ffi::CStr;
use std::sync::mpsc::Sender;

use block::*;

/*
// Values of call enum
use {
    es_event_type_t_ES_EVENT_TYPE_AUTH_EXEC as ES_EVENT_AUTH_EXEC,
    es_event_type_t_ES_EVENT_TYPE_AUTH_OPEN as ES_EVENT_AUTH_OPEN,
    es_event_type_t_ES_EVENT_TYPE_NOTIFY_EXEC as ES_EVENT_NOTIFY_EXEC,
    es_event_type_t_ES_EVENT_TYPE_NOTIFY_OPEN as ES_EVENT_NOTIFY_OPEN,
    //es_event_type_t_ES_EVENT_TYPE_NOTIFY_FORK as ES_EVENT_NOTIFY_FORK,
};
*/
// Values
use {
    es_return_t_ES_RETURN_SUCCESS as ES_RETURN_SUCCESS,
    
    //es_result_type_t_ES_RESULT_TYPE_AUTH as ES_RESULT_TYPE_AUTH,
    //es_result_type_t_ES_RESULT_TYPE_FLAGS as ES_RESULT_TYPE_FLAGS,
    
    es_auth_result_t_ES_AUTH_RESULT_ALLOW as ES_AUTH_RESULT_ALLOW,
    es_auth_result_t_ES_AUTH_RESULT_DENY as ES_AUTH_RESULT_DENY,
    
    es_action_type_t_ES_ACTION_TYPE_AUTH as ES_ACTION_TYPE_AUTH,
    es_action_type_t_ES_ACTION_TYPE_NOTIFY as ES_ACTION_TYPE_NOTIFY,

    es_new_client_result_t_ES_NEW_CLIENT_RESULT_SUCCESS as ES_NEW_CLIENT_SUCCESS,
    es_new_client_result_t_ES_NEW_CLIENT_RESULT_ERR_INVALID_ARGUMENT as ES_NEW_CLIENT_ERROR_INVALID_ARGUMENT,
    es_new_client_result_t_ES_NEW_CLIENT_RESULT_ERR_INTERNAL as ES_NEW_CLIENT_ERROR_INTERNAL,
    es_new_client_result_t_ES_NEW_CLIENT_RESULT_ERR_NOT_ENTITLED as ES_NEW_CLIENT_ERROR_NOT_ENTITLED,
    es_new_client_result_t_ES_NEW_CLIENT_RESULT_ERR_NOT_PERMITTED as ES_NEW_CLIENT_ERROR_NOT_PERMITTED,
    es_new_client_result_t_ES_NEW_CLIENT_RESULT_ERR_NOT_PRIVILEGED as ES_NEW_CLIENT_ERROR_NOT_PRIVILEGED,
    es_new_client_result_t_ES_NEW_CLIENT_RESULT_ERR_TOO_MANY_CLIENTS as ES_NEW_CLIENT_ERROR_TOO_MANY_CLIENTS,

    es_respond_result_t_ES_RESPOND_RESULT_SUCCESS as ES_RESPOND_RESULT_SUCCESS,
    es_respond_result_t_ES_RESPOND_RESULT_ERR_INVALID_ARGUMENT as ES_RESPONSE_RESULT_ERROR_INVALID_ARGUMENT,
    es_respond_result_t_ES_RESPOND_RESULT_ERR_INTERNAL as ES_RESPOND_RESULT_ERROR_INTERNAL,
    es_respond_result_t_ES_RESPOND_RESULT_NOT_FOUND as ES_RESPOND_RESULT_NOT_FOUND,
    es_respond_result_t_ES_RESPOND_RESULT_ERR_DUPLICATE_RESPONSE as ES_RESPOND_RESULT_ERROR_DUPLICATE_RESPONSE,
    es_respond_result_t_ES_RESPOND_RESULT_ERR_EVENT_TYPE as ES_RESPONSE_RESULT_ERROR_EVENT_TYPE,
};

pub struct EsFile {
    pub path: String,
    pub path_truncated: bool,
//    pub stat: stat,
}

pub struct EsProcess {
    //pub audit_token: rust_audit_token,
    pub ppid: u32,
    pub original_ppid: u32,
    pub group_id: u32,
    pub session_id: u32,
    pub codesigning_flags: u32,
    pub is_platform_binary: bool,
    pub is_es_client: bool,
    pub cdhash: [u8; 20usize],
    pub signing_id: String,
    pub team_id: String,
    pub executable: EsFile,
    //pub tty: EsFile,
    //pub start_time: timeval,
}

pub struct EsEventExec {
    pub target: EsProcess,
    pub args: String,
    // __bindgen_anon_1: es_event_exec_t__bindgen_ty_1,
}

pub struct EsEventOpen {
    pub fflag: u32,
    pub file: EsFile,
    // reserved: [u8; 64usize],
}

pub struct EsEventSignal {
    pub signal: i32,
    pub target: EsProcess,
    //pub reserved: [u8; 64usize],
}

pub struct EsEventLink {
    pub source: EsFile,
    pub target_dir: EsFile,
    pub target_filename: String,
    //pub reserved: [u8; 64usize],
}

pub struct EsEventUnlink {
    pub target: EsFile,
    pub parent_dir: EsFile,
    //pub reserved: [u8; 64usize],
}

pub enum EsDestinationType {
    ExistingFile = 0,
    NewPath = 1,
    Unknown = 2,
}

pub struct EsRenameDestinationNewPath {
    pub dir: EsFile,
    pub filename: String,
}

pub struct EsRenameDestination {
    pub existing_file: EsFile,
    pub new_path: EsRenameDestinationNewPath,
}

pub struct EsEventRename {
    pub source: EsFile,
    pub destination_type: EsDestinationType,
    pub destination: EsRenameDestination,
    //pub reserved: [u8; 64usize],
}

pub struct EsEventReadDir {
    pub target: EsFile,
}

pub enum EsRespondResult {
    Sucess,
    ErrorInvalidArgument,
    ErrorInternal,
    NotFound,
    ErrorDuplicateResponse,
    ErrorEventType,
    UnknownResponse,
}

pub enum EsNewClientResult {
    Success(EsClient),
    ErrorInvalidArgument(String),
    ErrorInternal(String),
    ErrorNotEntitled(String),
    ErrorNotPermitted(String),
    ErrorNotPrivileged(String),
    ErrorTooManyClients(String),
    Unknown(String),
}

#[repr(C)]
pub enum SupportedEsEvent {
    AuthExec = 0,
    AuthOpen = 1,
    AuthRename = 6,
    AuthSignal = 7,
    AuthUnlink = 8,
    NotifyExec = 9,
    NotifyOpen = 10,
    NotifyLink = 19,
    NotifyRename = 25,
    NotifySignal = 31,
    NotifyUnlink = 32,
    AuthLink = 42,
    AuthReadDir = 67,
    NotifyReadDir = 68,
}

pub enum EsEvent {
    AuthExec(EsEventExec),
    AuthOpen(EsEventOpen),
    /*AuthKextload,
    AuthMmap,
    AuthMprotect,
    AuthMount,*/
    AuthRename(EsEventRename),
    AuthSignal(EsEventSignal),
    AuthUnlink(EsEventUnlink),
    NotifyExec(EsEventExec),
    NotifyOpen(EsEventOpen),
    /*NotifyFork,
    NotifyClose,
    NotifyCreate,
    NotifyExchangedata,
    NotifyExit,
    NotifyGetTask,
    NotifyKextload,
    NotifyKextunload,*/
    NotifyLink(EsEventLink),
    /*NotifyMmap,
    NotifyMprotect,
    NotifyMount,
    NotifyUnmount,
    NotifyIokitOpen,*/
    NotifyRename(EsEventRename),
    /*NotifySetattrlist,
    NotifySetextattr,
    NotifySetflags,
    NotifySetmode,
    NotifySetowner,*/
    NotifySignal(EsEventSignal),
    NotifyUnlink(EsEventUnlink),
    /*NotifyWrite,
    AuthFileProviderMaterialize,
    NotifyFileProviderMaterialize,
    AuthFileProviderUpdate,
    NotifyFileProviderUpdate,
    AuthReadlink,
    NotifyReadlink,
    AuthTruncate,
    NotifyTruncate,*/
    AuthLink(EsEventLink),
    /*NotifyLookup,
    AuthCreate,
    AuthSetattrlist,
    AuthSetextattr,
    AuthSetflags,
    AuthSetmode,
    AuthSetowner,
    AuthChdir,
    NotifyChdir,
    AuthGetattrlist,
    NotifyGetattrlist,
    NotifyStat,
    NotifyAccess,
    AuthChroot,
    NotifyChroot,
    AuthUtimes,
    NotifyUtimes,
    AuthClone,
    NotifyClone,
    NotifyFcntl,
    AuthGetextattr,
    NotifyGetextattr,
    AuthListextattr,
    NotifyListextattr,*/
    AuthReadDir(EsEventReadDir),
    NotifyReadDir(EsEventReadDir),
    /*AuthDeleteextattr,
    NotifyDeleteextattr,
    AuthFsgetpath,
    NotifyFsgetpath,
    NotifyDup,
    AuthSettime,
    NotifySettime,
    NotifyUipcBind,
    AuthUipcBind,
    NotifyUipcConnect,
    AuthUipcConnect,
    AuthExchangedata,
    AuthSetacl,
    NotifySetacl,
    NotifyPtyGrant,
    NotifyPtyClose,
    AuthProcCheck,
    NotifyProcCheck,
    AuthGetTask,
    AuthSearchfs,
    NotifySearchfs,
    AuthFcntl,
    AuthIokitOpen,
    AuthProcSuspendResume,
    NotifyProcSuspendResume,
    NotifyCsInvalidated,
    NotifyGetTaskName,
    NotifyTrace,
    NotifyRemoteThreadCreate,
    AuthRemount,
    NotifyRemount,
    Last,*/
}

pub enum EsCacheResult {
    Yes,
    No,
}

pub enum EsActionType {
    Auth,
    Notify,
}

pub enum EsAuthResult {
    Allow,
    Deny,
}
/*
enum EsResultType {
    Auth,
    Flags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EsEventId {
    pub reserved: [u8; 32usize],
}

pub struct EsResultResult {
    auth: EsAuthResult,
    flags: u32,
    reserved: [u8; 32usize],
}

pub struct EsResult {
    result_type: EsResultType,
    result: EsResultResult,
}

pub struct EsAction {
    auth: EsEventId,
    notify: EsResult,
}*/

pub struct EsMessage {
    pub version: u32,
    pub time: u64,
    pub mach_time: u64,
    pub deadline: u64,
    pub process: EsProcess,
    pub seq_num: u64,
    pub action_type: EsActionType,
    //action: EsAction,
    pub event: EsEvent,
    raw_message: *const es_message_t,
}

pub struct EsClient {
    client: *mut es_client_t,
}

/// TODO: Codegen these in the future
fn raw_event_to_supportedesevent(event_type: u64) -> Option<SupportedEsEvent> {
    Some(match event_type {
        0 => SupportedEsEvent::AuthExec,
        1 => SupportedEsEvent::AuthOpen,
        7 => SupportedEsEvent::AuthSignal,
        8 => SupportedEsEvent::AuthUnlink,
        9 => SupportedEsEvent::NotifyExec,
        10 => SupportedEsEvent::NotifyOpen,
        19 => SupportedEsEvent::NotifyLink,
        31 => SupportedEsEvent::NotifySignal,
        32 => SupportedEsEvent::NotifyUnlink,
        42 => SupportedEsEvent::AuthLink,
        67 => SupportedEsEvent::AuthReadDir,
        68 => SupportedEsEvent::NotifyReadDir,
        _ => return None
    })
}

fn supportedesevent_to_raw_event(event_type: &SupportedEsEvent) -> u32 {
    match event_type {
        SupportedEsEvent::AuthExec => 0,
        SupportedEsEvent::AuthOpen => 1,
        SupportedEsEvent::AuthRename => 6,
        SupportedEsEvent::AuthSignal => 7,
        SupportedEsEvent::AuthUnlink => 8,
        SupportedEsEvent::NotifyExec => 9,
        SupportedEsEvent::NotifyOpen => 10,
        SupportedEsEvent::NotifyLink => 19,
        SupportedEsEvent::NotifyRename => 25,
        SupportedEsEvent::NotifySignal => 31,
        SupportedEsEvent::NotifyUnlink => 32,
        SupportedEsEvent::AuthLink => 42,
        SupportedEsEvent::AuthReadDir => 67,
        SupportedEsEvent::NotifyReadDir => 68,
    }
}

fn parse_es_file_ptr(file: *mut es_file_t) -> EsFile {
    unsafe {
        let file = *file;

        EsFile {
            path: CStr::from_ptr(file.path.data).to_str().unwrap().to_owned(),
            path_truncated: { file.path_truncated },
        }
    }
}

fn parse_es_file(file: &es_file_t) -> EsFile {
    EsFile {
        path: unsafe { CStr::from_ptr(file.path.data).to_str().unwrap().to_owned() },
        path_truncated: { file.path_truncated },
    }
}

fn parse_c_string(string_token: es_string_token_t) -> String {
    match string_token.length {
        x if x <= 0 => {
            String::new()
        },
        _ => {
            match unsafe { CStr::from_ptr(string_token.data).to_str() }{
                Ok(v) => v.to_owned(),
                Err(e) => {
                    println!("String would not parse: {}", e);
                    String::new()
                }
            }
        },
    }
}

fn parse_es_process(process: &es_process_t) -> EsProcess {
    EsProcess {
        ppid: process.ppid as u32,
        original_ppid: process.original_ppid as u32,
        group_id: process.group_id as u32,
        session_id: process.session_id as u32,
        codesigning_flags: process.codesigning_flags as u32,
        is_platform_binary: process.is_platform_binary,
        is_es_client: process.is_es_client,
        cdhash: process.cdhash,
        signing_id: parse_c_string(process.signing_id),
        team_id: parse_c_string(process.team_id),
        executable: parse_es_file_ptr(process.executable),
    }
}

fn parse_es_event(event_type: SupportedEsEvent, event: es_events_t, action_type: &EsActionType) -> EsEvent {
    unsafe {
        match event_type {
            SupportedEsEvent::AuthExec | SupportedEsEvent::NotifyExec => {
                let target = event.exec.target;
                let event = EsEventExec {
                    target: parse_es_process(&*target),
                    args: String::new(),
                };

                match action_type {
                    EsActionType::Notify => EsEvent::NotifyExec(event),
                    EsActionType::Auth => EsEvent::AuthExec(event),
                }
            },
            SupportedEsEvent::AuthOpen | SupportedEsEvent::NotifyOpen => {
                let file = event.open;
                let event = EsEventOpen {
                    fflag: file.fflag as u32,
                    file: parse_es_file_ptr(file.file),
                };
                match action_type {
                    EsActionType::Notify => EsEvent::NotifyOpen(event),
                    EsActionType::Auth => EsEvent::AuthOpen(event),
                }
            },
            SupportedEsEvent::AuthSignal | SupportedEsEvent::NotifySignal => {
                let target = event.signal.target;
                let event = EsEventSignal {
                    signal: event.signal.sig,
                    target: parse_es_process(&*target),
                };
                match action_type {
                    EsActionType::Notify => EsEvent::NotifySignal(event),
                    EsActionType::Auth => EsEvent::AuthSignal(event),
                }
            },
            SupportedEsEvent::AuthUnlink | SupportedEsEvent::NotifyUnlink => {
                let target = event.unlink.target;
                let parent_dir = event.unlink.target;
                let event = EsEventUnlink {
                    target: parse_es_file(&*target),
                    parent_dir: parse_es_file(&*parent_dir),
                };
            match action_type {
                    EsActionType::Notify => EsEvent::NotifyUnlink(event),
                    EsActionType::Auth => EsEvent::AuthUnlink(event),
                }
            },
            SupportedEsEvent::AuthLink | SupportedEsEvent::NotifyLink => {
                let event = EsEventLink {
                    source: parse_es_file_ptr(event.link.source),
                    target_dir: parse_es_file_ptr(event.link.target_dir),
                    target_filename: parse_c_string(event.link.target_filename),
                };
                match action_type {
                    EsActionType::Notify => EsEvent::NotifyLink(event),
                    EsActionType::Auth => EsEvent::AuthLink(event),
                }
            },
            SupportedEsEvent::AuthRename | SupportedEsEvent::NotifyRename => {
                let event = EsEventRename {
                    source: parse_es_file_ptr(event.rename.source),
                    destination_type: match event.rename.destination_type {
                        0 => EsDestinationType::ExistingFile,
                        1 => EsDestinationType::NewPath,
                        _ => EsDestinationType::Unknown,
                    },
                    destination: EsRenameDestination {
                        existing_file: parse_es_file_ptr(event.rename.destination.existing_file),
                        new_path: EsRenameDestinationNewPath {
                            dir: parse_es_file_ptr(event.rename.destination.new_path.dir),
                            filename: parse_c_string(event.rename.destination.new_path.filename),
                        },
                    }
                };
                match action_type {
                    EsActionType::Notify => EsEvent::NotifyRename(event),
                    EsActionType::Auth => EsEvent::AuthRename(event),
                }
            },
            SupportedEsEvent::AuthReadDir | SupportedEsEvent::NotifyReadDir => {
                let event = EsEventReadDir {
                    target: parse_es_file_ptr(event.readdir.target),
                };
                match action_type {
                    EsActionType::Notify => EsEvent::NotifyReadDir(event),
                    EsActionType::Auth => EsEvent::NotifyReadDir(event),
                }
            },
        }
    }
}

fn parse_es_message(message: *mut es_message_t) -> Result<EsMessage, &'static str> {
    unsafe {
        let message = &*message;
        let process = &*(message.process);
        let action_type = match message.action_type {
            ES_ACTION_TYPE_AUTH => EsActionType::Auth,
            ES_ACTION_TYPE_NOTIFY => EsActionType::Notify,
            _ => return Err("Couldn't parse action_type"), // At time of writing these are the only two ways
        };

        let event = if let Some(event) = raw_event_to_supportedesevent(message.event_type as u64) {
            parse_es_event(event, message.event, &action_type)
        } else {
            return Err("Could not parse this event type");
        };

        Ok(EsMessage {
            version: message.version,
            time: message.time.tv_nsec as u64,
            mach_time: message.mach_time,
            deadline: message.deadline,
            process: parse_es_process(process),
            seq_num: message.seq_num,
            action_type: action_type,
            // This is a union field. Probably needs to be handled with an enum
            // Maybe leaks kernel memory because notify is bigger than auth?
            /*action: EsAction {
                auth: EsEventId{
                    reserved: message.action.auth.reserved,
                },
                notify: EsResult {
                    result_type: {
                        println!("The value is: {}", message.action.notify.result_type);
                        match message.action.notify.result_type {
                            ES_RESULT_TYPE_AUTH => EsResultType::Auth,
                            ES_RESULT_TYPE_FLAGS => EsResultType::Flags,
                            _ => return Err("Couldn't parse action notify result_type"),   // At time of writing these are the only types
                        }
                    },
                    result: EsResultResult {
                        auth: {
                            match message.action.notify.result.auth {
                                ES_AUTH_RESULT_ALLOW => EsAuthResult::Allow,
                                ES_AUTH_RESULT_DENY => EsAuthResult::Deny,
                                _ => return Err("Couldn't parse action notify result"), // At time of writing these are the only two
                            }
                        },
                        flags: message.action.notify.result.flags,
                        reserved: message.action.notify.result.reserved,
                    }
                }
            },*/
            event: event,
            raw_message: message,
        })
    }
}

extern fn es_notify_callback(_client: *mut es_client_t, message: *mut es_message_t, tx: Sender<EsMessage>) {
    let message = match parse_es_message(message) {
        Err(e) => { println!("Could not parse message: {}", e); return},
        Ok(x) => x,
    };

    match tx.send(message) {
        Err(e) => println!("Error logging event: {}", e),
        _ => (),
    }
}

pub fn create_es_client(tx: Sender<EsMessage>) -> EsNewClientResult {
    let mut client: *mut es_client_t = std::ptr::null_mut();
    let client_ptr: *mut *mut es_client_t = &mut client;

    let handler = ConcreteBlock::new(move |client: *mut es_client_t, message: *mut es_message_t| {
        es_notify_callback(client, message, tx.clone());
    }).copy();

    match unsafe { es_new_client(client_ptr, &*handler as *const Block<_, _> as *const std::ffi::c_void) } {
        ES_NEW_CLIENT_SUCCESS => EsNewClientResult::Success(EsClient {client: client}),
        ES_NEW_CLIENT_ERROR_INVALID_ARGUMENT => EsNewClientResult::ErrorInvalidArgument(String::from("Something incorrect was passed to es_new_client")),
        ES_NEW_CLIENT_ERROR_INTERNAL => EsNewClientResult::ErrorInternal(String::from("es_new_client experienced an internal error")),
        ES_NEW_CLIENT_ERROR_NOT_ENTITLED => EsNewClientResult::ErrorNotEntitled(String::from("This process doesn't have the EndpointSecurity entitlement")),
        ES_NEW_CLIENT_ERROR_NOT_PERMITTED => EsNewClientResult::ErrorNotPermitted(String::from("This process is not permitted to use the EndpointSecurity Framework")),
        ES_NEW_CLIENT_ERROR_NOT_PRIVILEGED => EsNewClientResult::ErrorNotPrivileged(String::from("The process must be running as root to access the EndpointSecurity Framework")),
        ES_NEW_CLIENT_ERROR_TOO_MANY_CLIENTS => EsNewClientResult::ErrorTooManyClients(String::from("There are too many clients connected to EndpointSecurity")),
        _ => EsNewClientResult::Unknown(String::from("es_new_client returned an unknown error")),
    }
}

pub fn subscribe_to_events(client: &EsClient, events: &Vec<SupportedEsEvent>) -> bool {
    if events.len() > 128 {
        println!("Too many events to subscribe to!");
        return false;
    }

    let mut c_events: [u32; 128] = [0; 128];
    let mut i = 0;
    for event in events {
        c_events[i] = supportedesevent_to_raw_event(&*event);
        i += 1;
    }

    unsafe {
        match es_subscribe(client.client, &c_events as *const u32, events.len() as u32) {
            ES_RETURN_SUCCESS => true,
            _ => false,
        }
    }
}

pub fn respond_to_flags_event(client: &EsClient, message: &EsMessage, authorized_flags: u32, should_cache: &EsCacheResult) -> EsRespondResult {
    let cache = match should_cache {
        EsCacheResult::Yes => true,
        EsCacheResult::No => false,
    };

    match  unsafe { es_respond_flags_result(client.client, message.raw_message, authorized_flags, cache) } {
        ES_RESPOND_RESULT_SUCCESS => EsRespondResult::Sucess,
        ES_RESPONSE_RESULT_ERROR_INVALID_ARGUMENT => EsRespondResult::ErrorInvalidArgument,
        ES_RESPOND_RESULT_ERROR_INTERNAL => EsRespondResult::ErrorInternal,
        ES_RESPOND_RESULT_NOT_FOUND => EsRespondResult::NotFound,
        ES_RESPOND_RESULT_ERROR_DUPLICATE_RESPONSE => EsRespondResult::ErrorDuplicateResponse,
        ES_RESPONSE_RESULT_ERROR_EVENT_TYPE => EsRespondResult::ErrorEventType,
        _ => EsRespondResult::UnknownResponse,
    }
}

pub fn respond_to_auth_event(client: &EsClient, message: &EsMessage, response: &EsAuthResult, should_cache: &EsCacheResult) -> EsRespondResult {
    let cache = match should_cache {
        EsCacheResult::Yes => true,
        EsCacheResult::No => false,
    };

    let response = match response {
        EsAuthResult::Allow => ES_AUTH_RESULT_ALLOW,
        EsAuthResult::Deny => ES_AUTH_RESULT_DENY,
    };

   
    match  unsafe { es_respond_auth_result(client.client, message.raw_message, response, cache) } {
        ES_RESPOND_RESULT_SUCCESS => EsRespondResult::Sucess,
        ES_RESPONSE_RESULT_ERROR_INVALID_ARGUMENT => EsRespondResult::ErrorInvalidArgument,
        ES_RESPOND_RESULT_ERROR_INTERNAL => EsRespondResult::ErrorInternal,
        ES_RESPOND_RESULT_NOT_FOUND => EsRespondResult::NotFound,
        ES_RESPOND_RESULT_ERROR_DUPLICATE_RESPONSE => EsRespondResult::ErrorDuplicateResponse,
        ES_RESPONSE_RESULT_ERROR_EVENT_TYPE => EsRespondResult::ErrorEventType,
        _ => EsRespondResult::UnknownResponse,
    }
}
