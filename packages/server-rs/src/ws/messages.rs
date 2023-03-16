use super::{Client, LocaleValue};
use log::warn;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use std::{fmt::Display, str::FromStr};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageArgs<T> {
    pub id: String,
    pub connId: String,
    pub r#type: MessageType,
    pub data: T,
}

impl<T> Display for MessageArgs<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use webrtc::{
    ice_transport::{
        ice_candidate::RTCIceCandidate, ice_candidate_type::RTCIceCandidateType,
        ice_protocol::RTCIceProtocol,
    },
    peer_connection::sdp::session_description::RTCSessionDescription,
};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum MessageType {
    SET_LOCALE,
    GET_LOCALE,
    GET_USER_ID,
    SET_USER_ID,
    GET_LOGIN,
    TOKEN,
    OFFER,
    CANDIDATE,
    ANSWER,
    GET_ROOM,
    SET_ROOM,
    GET_CHAT_UNIT,
    GET_SETTINGS_UNIT,
    SET_ERROR,
    GET_ROOM_GUESTS,
    SET_ROOM_GUESTS,
    SET_CHANGE_UNIT,
    GET_MUTE,
    SET_MUTE,
    GET_NEED_RECONNECT,
    GET_CLOSE_PEER_CONNECTION,
    SET_CLOSE_PEER_CONNECTION,
    GET_ROOM_MESSAGE,
    SET_ROOM_MESSAGE,
    SET_CHAT_UNIT,
    SET_SETTINGS_UNIT,
    GET_CHAT_MESSAGES,
    SET_CHAT_MESSAGES,
    GET_EDIT_MESSAGE,
    SET_EDIT_MESSAGE,
    GET_CREATE_MESSAGE,
    SET_CREATE_MESSAGE,
    GET_CREATE_QUOTE,
    SET_CREATE_QUOTE,
    GET_DELETE_MESSAGE,
    SET_DELETE_MESSAGE,
    GET_TO_MUTE,
    GET_TO_BAN,
    GET_TO_UNMUTE,
    GET_TO_UNBAN,
    SET_BAN_LIST,
    SET_MUTE_LIST,
    GET_RECORD,
    SET_RECORDING,
    GET_VIDEO_FIND_MANY,
    SET_VIDEO_FIND_MANY,
    GET_VIDEO_FIND_FIRST,
    SET_VIDEO_FIND_FIRST,
    GET_ASK_FLOOR,
    SET_ASK_FLOOR,
    GET_MUTE_FOR_ALL,
    SET_MUTE_FOR_ALL,
    GET_BLOCK_CHAT,
    SET_BLOCK_CHAT,
    GET_VIDEO_TRACK,
    SET_VIDEO_TRACK,
    GET_TO_ADMIN,
    SET_TO_ADMIN,
    GET_VIDEO_SETTINGS,
    SET_CREATE_VIDEO,
    GET_VIDEO_DELETE,
    SET_VIDEO_DELETE,
    GET_VIDEO_UPDATE,
    SET_VIDEO_UPDATE,
}

impl FromStr for MessageType {
    type Err = ();
    fn from_str(input: &str) -> Result<MessageType, ()> {
        match input {
            "GET_LOCALE" => Ok(MessageType::GET_LOCALE),
            "SET_LOCALE" => Ok(MessageType::SET_LOCALE),
            "GET_USER_ID" => Ok(MessageType::GET_USER_ID),
            "SET_USER_ID" => Ok(MessageType::SET_USER_ID),
            "GET_LOGIN" => Ok(MessageType::GET_LOGIN),
            "TOKEN" => Ok(MessageType::TOKEN),
            "OFFER" => Ok(MessageType::OFFER),
            "ANSWER" => Ok(MessageType::ANSWER),
            "CANDIDATE" => Ok(MessageType::CANDIDATE),
            "GET_ROOM" => Ok(MessageType::GET_ROOM),
            "SET_ROOM" => Ok(MessageType::SET_ROOM),
            "GET_CHAT_UNIT" => Ok(MessageType::GET_CHAT_UNIT),
            "GET_SETTINGS_UNIT" => Ok(MessageType::GET_SETTINGS_UNIT),
            "SET_ERROR" => Ok(MessageType::SET_ERROR),
            "GET_ROOM_GUESTS" => Ok(MessageType::GET_ROOM_GUESTS),
            "SET_ROOM_GUESTS" => Ok(MessageType::SET_ROOM_GUESTS),
            "SET_CHANGE_UNIT" => Ok(MessageType::SET_CHANGE_UNIT),
            "GET_MUTE" => Ok(MessageType::GET_MUTE),
            "GET_NEED_RECONNECT" => Ok(MessageType::GET_NEED_RECONNECT),
            "SET_MUTE" => Ok(MessageType::SET_MUTE),
            "GET_CLOSE_PEER_CONNECTION" => Ok(MessageType::GET_CLOSE_PEER_CONNECTION),
            "SET_CLOSE_PEER_CONNECTION" => Ok(MessageType::SET_CLOSE_PEER_CONNECTION),
            "GET_ROOM_MESSAGE" => Ok(MessageType::GET_ROOM_MESSAGE),
            "SET_ROOM_MESSAGE" => Ok(MessageType::SET_ROOM_MESSAGE),
            "SET_CHAT_UNIT" => Ok(MessageType::SET_CHAT_UNIT),
            "SET_SETTINGS_UNIT" => Ok(MessageType::SET_SETTINGS_UNIT),
            "GET_CHAT_MESSAGES" => Ok(MessageType::GET_CHAT_MESSAGES),
            "SET_CHAT_MESSAGES" => Ok(MessageType::SET_CHAT_MESSAGES),
            "GET_EDIT_MESSAGE" => Ok(MessageType::GET_EDIT_MESSAGE),
            "SET_EDIT_MESSAGE" => Ok(MessageType::SET_EDIT_MESSAGE),
            "GET_CREATE_MESSAGE" => Ok(MessageType::GET_CREATE_MESSAGE),
            "SET_CREATE_MESSAGE" => Ok(MessageType::SET_CREATE_MESSAGE),
            "GET_CREATE_QUOTE" => Ok(MessageType::GET_CREATE_QUOTE),
            "SET_CREATE_QUOTE" => Ok(MessageType::SET_CREATE_QUOTE),
            "GET_DELETE_MESSAGE" => Ok(MessageType::GET_DELETE_MESSAGE),
            "SET_DELETE_MESSAGE" => Ok(MessageType::SET_DELETE_MESSAGE),
            "GET_TO_MUTE" => Ok(MessageType::GET_TO_MUTE),
            "GET_TO_BAN" => Ok(MessageType::GET_TO_BAN),
            "GET_TO_UNMUTE" => Ok(MessageType::GET_TO_UNMUTE),
            "GET_TO_UNBAN" => Ok(MessageType::GET_TO_UNBAN),
            "SET_BAN_LIST" => Ok(MessageType::SET_BAN_LIST),
            "SET_MUTE_LIST" => Ok(MessageType::SET_MUTE_LIST),
            "GET_RECORD" => Ok(MessageType::GET_RECORD),
            "SET_RECORDING" => Ok(MessageType::SET_RECORDING),
            "GET_VIDEO_FIND_MANY" => Ok(MessageType::GET_VIDEO_FIND_MANY),
            "SET_VIDEO_FIND_MANY" => Ok(MessageType::SET_VIDEO_FIND_MANY),
            "GET_VIDEO_FIND_FIRST" => Ok(MessageType::GET_VIDEO_FIND_FIRST),
            "SET_VIDEO_FIND_FIRST" => Ok(MessageType::SET_VIDEO_FIND_FIRST),
            "GET_ASK_FLOOR" => Ok(MessageType::GET_ASK_FLOOR),
            "SET_ASK_FLOOR" => Ok(MessageType::SET_ASK_FLOOR),
            "GET_MUTE_FOR_ALL" => Ok(MessageType::GET_MUTE_FOR_ALL),
            "SET_MUTE_FOR_ALL" => Ok(MessageType::SET_MUTE_FOR_ALL),
            "GET_BLOCK_CHAT" => Ok(MessageType::GET_BLOCK_CHAT),
            "SET_BLOCK_CHAT" => Ok(MessageType::SET_BLOCK_CHAT),
            "GET_VIDEO_TRACK" => Ok(MessageType::GET_VIDEO_TRACK),
            "SET_VIDEO_TRACK" => Ok(MessageType::SET_VIDEO_TRACK),
            "GET_TO_ADMIN" => Ok(MessageType::GET_TO_ADMIN),
            "SET_TO_ADMIN" => Ok(MessageType::SET_TO_ADMIN),
            "GET_VIDEO_SETTINGS" => Ok(MessageType::GET_VIDEO_SETTINGS),
            "SET_CREATE_VIDEO" => Ok(MessageType::SET_CREATE_VIDEO),
            "GET_VIDEO_DELETE" => Ok(MessageType::GET_VIDEO_DELETE),
            "SET_VIDEO_DELETE" => Ok(MessageType::SET_VIDEO_DELETE),
            "GET_VIDEO_UPDATE" => Ok(MessageType::GET_VIDEO_UPDATE),
            "SET_VIDEO_UPDATE" => Ok(MessageType::SET_VIDEO_UPDATE),
            _ => {
                warn!("Default case of MessageType: {}", input);
                Err(())
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RoomList {
    pub room_id: String,
    pub users: Vec<String>,
}

pub trait FromValue {
    fn from(value: &Value) -> Self;
}

#[derive(Debug)]
pub struct GetLocale {
    pub locale: LocaleValue,
}

impl FromValue for GetLocale {
    fn from(value: &Value) -> Self {
        Self {
            locale: LocaleValue::from_str(value["locale"].as_str().unwrap()).unwrap(),
        }
    }
}

#[derive(Serialize)]
pub struct SetLocale {
    pub locale: Client<'static>,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct GetSettingsUnit {
    pub userId: String,
    pub locale: LocaleValue,
}

pub type SetSettingsUnit = ();

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct GetChatUnit {
    pub userId: String,
    pub locale: LocaleValue,
}

impl FromValue for GetChatUnit {
    fn from(value: &Value) -> Self {
        Self {
            userId: value["userId"].as_str().unwrap().to_string(),
            locale: LocaleValue::from_str(value["locale"].as_str().unwrap()).unwrap(),
        }
    }
}

pub type SetChatUnit = ();

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct GetUserId {
    pub isRoom: Option<bool>,
    pub userName: String,
    pub locale: LocaleValue,
}

impl FromValue for GetUserId {
    fn from(value: &Value) -> Self {
        Self {
            isRoom: value["isRoom"].as_bool(),
            userName: value["userName"].as_str().unwrap().to_string(),
            locale: LocaleValue::from_str(value["locale"].as_str().unwrap()).unwrap(),
        }
    }
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct SetUserId {
    pub name: String,
}

pub type Any = ();

impl FromValue for Any {
    fn from(_: &Value) -> Self {
        ()
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct GetRoom {
    pub userId: String,
    pub mimeType: String,
    pub isPublic: bool,
}

impl FromValue for GetRoom {
    fn from(value: &Value) -> Self {
        Self {
            isPublic: value["isPublic"].as_bool().unwrap(),
            mimeType: value["mimeType"].as_str().unwrap().to_string(),
            userId: value["userId"].as_str().unwrap().to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct SetRoom {
    pub isOwner: bool,
    pub asked: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct _RTCSessionDescription {
    pub r#type: String,
    pub sdp: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Offer {
    pub sdp: RTCSessionDescription,
    pub userId: String,
    pub target: String,
    pub mimeType: String,
    pub roomId: String,
}

impl FromValue for Offer {
    fn from(value: &Value) -> Self {
        Self {
            sdp: from_str::<RTCSessionDescription>(
                to_string(&_RTCSessionDescription {
                    sdp: value["sdp"]["sdp"].as_str().unwrap().to_string(),
                    r#type: value["sdp"]["type"].as_str().unwrap().to_string(),
                })
                .unwrap()
                .as_str(),
            )
            .unwrap(),
            userId: value["userId"].as_str().unwrap().to_string(),
            mimeType: value["mimeType"].as_str().unwrap().to_string(),
            target: value["target"].as_str().unwrap().to_string(),
            roomId: value["roomId"].as_str().unwrap().to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Candidate {
    candidate: RTCIceCandidate,
    userId: String,
    target: String,
    roomId: String,
}

#[derive(Serialize, Debug)]
pub struct _RTCIceCandidate {
    pub stats_id: String,         // + usernameFragment
    pub foundation: String,       // +
    pub priority: u32,            // +
    pub address: String,          // +
    pub protocol: RTCIceProtocol, // +
    pub port: u16,                // +
    pub typ: RTCIceCandidateType, // + type
    pub component: u16,
    pub related_address: String, // +  ip
    pub related_port: u16,
    pub tcp_type: String,
    /*
    address: "192.168.0.3" // +
    candidate: "candidate:0 1 UDP 2122252543 192.168.0.3 58063 typ host"
    component: "rtp"
    foundation: "0" // +
    ip: "192.168.0.3" // + related_address
    port: 58063 // +
    priority: 2122252543 // +
    protocol: "udp" // +
    sdpMLineIndex: 0
    sdpMid: "0"
    toJSON: function toJSON()
    type: "host" // + typ
    usernameFragment: "b276b198" // + stats_id
    */
}

impl FromValue for Candidate {
    fn from(value: &Value) -> Self {
        let def_cand = RTCIceCandidate::default();

        let mut stats_id = def_cand.stats_id;
        if let Some(_) = value["candidate"]["usernameFragment"].as_str() {
            stats_id = value["candidate"]["usernameFragment"]
                .as_str()
                .unwrap()
                .to_string();
        }

        let mut foundation = def_cand.foundation;
        if let Some(_) = value["candidate"]["foundation"].as_str() {
            foundation = value["candidate"]["foundation"]
                .as_str()
                .unwrap()
                .to_string();
        }

        let mut address = def_cand.address;
        if let Some(_) = value["candidate"]["address"].as_str() {
            address = value["candidate"]["address"].as_str().unwrap().to_string();
        }

        let mut protocol = def_cand.protocol;
        if let Some(_) = value["candidate"]["protocol"].as_str() {
            protocol = RTCIceProtocol::from(value["candidate"]["protocol"].as_str().unwrap());
        }

        let mut port = def_cand.port;
        if let Some(_) = value["candidate"]["port"].as_str() {
            port = value["candidate"]["port"].as_u64().unwrap() as u16;
        }

        let mut priority = def_cand.priority;
        if let Some(_) = value["candidate"]["priority"].as_str() {
            priority = value["candidate"]["priority"].as_u64().unwrap() as u32;
        }

        let mut related_address = def_cand.related_address;
        if let Some(_) = value["candidate"]["ip"].as_str() {
            related_address = value["candidate"]["ip"].as_str().unwrap().to_string();
        }

        let mut component = def_cand.component;
        if let Some(_) = value["candidate"]["component"].as_str() {
            component = 2;
            if value["candidate"]["component"].as_str().unwrap() == "rtp" {
                component = 1;
            }
        }

        let mut related_port = def_cand.related_port;
        if let Some(_) = value["candidate"]["port"].as_str() {
            related_port = value["candidate"]["port"].as_u64().unwrap() as u16;
        }

        let mut tcp_type = def_cand.tcp_type;
        if let Some(_) = value["candidate"]["tcpType"].as_str() {
            tcp_type = value["candidate"]["tcpType"].as_str().unwrap().to_string()
        }

        let mut typ = def_cand.typ;
        if let Some(_) = value["candidate"]["tcpType"].as_str() {
            typ = RTCIceCandidateType::from(value["candidate"]["type"].as_str().unwrap())
        }

        Self {
            candidate: from_str::<RTCIceCandidate>(
                to_string(&_RTCIceCandidate {
                    stats_id,
                    foundation,
                    address,
                    protocol,
                    port,
                    priority,
                    related_address,
                    component,
                    related_port,
                    tcp_type,
                    typ,
                })
                .unwrap()
                .as_str(),
            )
            .unwrap(),
            userId: value["userId"].as_str().unwrap().to_string(),
            target: value["target"].as_str().unwrap().to_string(),
            roomId: value["roomId"].as_str().unwrap().to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Answer {
    sdp: RTCSessionDescription,
    userId: String,
    target: String,
}

impl FromValue for Answer {
    fn from(value: &Value) -> Self {
        Self {
            sdp: from_str::<RTCSessionDescription>(
                to_string(&_RTCSessionDescription {
                    sdp: value["sdp"]["sdp"].as_str().unwrap().to_string(),
                    r#type: value["sdp"]["type"].as_str().unwrap().to_string(),
                })
                .unwrap()
                .as_str(),
            )
            .unwrap(),
            userId: value["userId"].as_str().unwrap().to_string(),
            target: value["target"].as_str().unwrap().to_string(),
        }
    }
}
