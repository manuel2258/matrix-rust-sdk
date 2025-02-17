use lazy_static::lazy_static;
use serde_json::{json, Value as JsonValue};

lazy_static! {
    pub static ref ALIAS: JsonValue = json!({
        "content": {
            "alias": "#tutorial:localhost"
        },
        "event_id": "$15139375513VdeRF:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.canonical_alias",
        "unsigned": {
            "age": 703422
        }
    });
}

lazy_static! {
    pub static ref ALIASES: JsonValue = json!({
        "content": {
            "aliases": [
                "#tutorial:localhost"
            ]
        },
        "event_id": "$15139375516NUgtD:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "localhost",
        "type": "m.room.aliases",
        "unsigned": {
            "age": 703422
        }
    });
}

lazy_static! {
    pub static ref CREATE: JsonValue = json!({
        "content": {
            "creator": "@example:localhost",
            "m.federate": true,
            "room_version": "1"
        },
        "event_id": "$151957878228ekrDs:localhost",
        "origin_server_ts": 15195787,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.create",
        "unsigned": {
          "age": 139298
        }
    });
}

lazy_static! {
    pub static ref FULLY_READ: JsonValue = json!({
        "content": {
            "event_id": "$someplace:example.org"
        },
        "room_id": "!somewhere:example.org",
        "type": "m.fully_read"
    });
}

lazy_static! {
    pub static ref HISTORY_VISIBILITY: JsonValue = json!({
        "content": {
            "history_visibility": "world_readable"
        },
        "event_id": "$151957878235ricnD:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.history_visibility",
        "unsigned": {
          "age": 1392989
        }
    });
}

lazy_static! {
    pub static ref JOIN_RULES: JsonValue = json!({
        "content": {
            "join_rule": "public"
        },
        "event_id": "$151957878231iejdB:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.join_rules",
        "unsigned": {
          "age": 1392989
        }
    });
}

lazy_static! {
    pub static ref ROOM_MESSAGES: JsonValue = json!({
        "chunk": [
          {
            "age": 1042,
            "content": {
              "body": "hello world",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Caaaa:example.com",
            "origin_server_ts": 1444812213737i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@alice:example.com",
            "type": "m.room.message"
          },
          {
            "age": 20123,
            "content": {
              "body": "the world is big",
              "msgtype": "m.text"
            },
            "event_id": "$1444812213350496Cbbbb:example.com",
            "origin_server_ts": 1444812194656i64,
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "type": "m.room.message"
          },
          {
            "age": 50789,
            "content": {
              "name": "New room name"
            },
            "event_id": "$1444812213350496Ccccc:example.com",
            "origin_server_ts": 1444812163990i64,
            "prev_content": {
              "name": "Old room name"
            },
            "room_id": "!Xq3620DUiqCaoxq:example.com",
            "sender": "@bob:example.com",
            "state_key": "",
            "type": "m.room.name"
          }
        ],
        "end": "t47409-4357353_219380_26003_2265",
        "start": "t47429-4392820_219380_26003_2265"
    });
}

lazy_static! {
    pub static ref KEYS_QUERY: JsonValue = json!({
      "device_keys": {
        "@alice:example.org": {
          "JLAFKJWSCS": {
              "algorithms": [
                  "m.olm.v1.curve25519-aes-sha2",
                  "m.megolm.v1.aes-sha2"
              ],
              "device_id": "JLAFKJWSCS",
              "user_id": "@alice:example.org",
              "keys": {
                  "curve25519:JLAFKJWSCS": "wjLpTLRqbqBzLs63aYaEv2Boi6cFEbbM/sSRQ2oAKk4",
                  "ed25519:JLAFKJWSCS": "nE6W2fCblxDcOFmeEtCHNl8/l8bXcu7GKyAswA4r3mM"
              },
              "signatures": {
                  "@alice:example.org": {
                      "ed25519:JLAFKJWSCS": "m53Wkbh2HXkc3vFApZvCrfXcX3AI51GsDHustMhKwlv3TuOJMj4wistcOTM8q2+e/Ro7rWFUb9ZfnNbwptSUBA"
                  }
              },
              "unsigned": {
                  "device_display_name": "Alice's mobile phone"
              }
          }
        }
      },
      "failures": {}
    });
}

lazy_static! {
    pub static ref KEYS_UPLOAD: JsonValue = json!({
      "one_time_key_counts": {
        "curve25519": 10,
        "signed_curve25519": 20
      }
    });
}

lazy_static! {
    pub static ref LOGIN: JsonValue = json!({
        "access_token": "abc123",
        "device_id": "GHTYAJCE",
        "home_server": "matrix.org",
        "user_id": "@cheeky_monkey:matrix.org"
    });
}

lazy_static! {
    pub static ref LOGIN_WITH_DISCOVERY: JsonValue = json!({
        "access_token": "abc123",
        "device_id": "GHTYAJCE",
        "home_server": "matrix.org",
        "user_id": "@cheeky_monkey:matrix.org",
        "well_known": {
            "m.homeserver": {
                "base_url": "https://example.org"
            },
            "m.identity_server": {
                "base_url": "https://id.example.org"
            }
        }
    });
}

lazy_static! {
    pub static ref LOGIN_RESPONSE_ERR: JsonValue = json!({
      "errcode": "M_FORBIDDEN",
      "error": "Invalid password"
    });
}

lazy_static! {
    pub static ref LOGIN_TYPES: JsonValue = json!({
        "flows": [
            {
                "type": "m.login.password"
            },
            {
                "type": "m.login.sso"
            },
            {
                "type": "m.login.token"
            }
        ]
    });
}

lazy_static! {
    pub static ref LOGOUT: JsonValue = json!({});
}

lazy_static! {
    pub static ref EVENT_ID: JsonValue = json!({
        "event_id": "$h29iv0s8:example.com"
    });
}

// TODO: Move `prev_content` into `unsigned` once ruma supports it
lazy_static! {
    pub static ref MEMBER: JsonValue = json!({
        "content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "join"
        },
        "event_id": "$151800140517rfvjc:localhost",
        "membership": "join",
        "origin_server_ts": 151800140,
        "sender": "@example:localhost",
        "state_key": "@example:localhost",
        "type": "m.room.member",
        "prev_content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "invite"
        },
        "unsigned": {
            "age": 297036,
            "replaces_state": "$151800111315tsynI:localhost"
        }
    });
}

lazy_static! {
    pub static ref MEMBER_INVITE: JsonValue = json!({
        "content": {
            "avatar_url": "mxc://localhost/SEsfnsuifSDFSSEF",
            "displayname": "example",
            "membership": "invite",
            "reason": "Looking for support"
        },
        "event_id": "$143273582443PhrSn:localhost",
        "origin_server_ts": 1432735824,
        "room_id": "!jEsUZKDJdhlrceRyVU:localhost",
        "sender": "@example:localhost",
        "state_key": "@invited:localhost",
        "type": "m.room.member",
        "unsigned": {
            "age": 1234,
            "invite_room_state": [
                {
                    "content": {
                        "name": "Example Room"
                    },
                    "sender": "@example:localhost",
                    "state_key": "",
                    "type": "m.room.name"
                },
                {
                    "content": {
                        "join_rule": "invite"
                    },
                    "sender": "@example:localhost",
                    "state_key": "",
                    "type": "m.room.join_rules"
                }
            ]
        }
    });
}

// TODO: Move `prev_content` into `unsigned` once ruma supports it
lazy_static! {
    pub static ref MEMBER_NAME_CHANGE: JsonValue = json!({
        "content": {
            "avatar_url": null,
            "displayname": "changed",
            "membership": "join"
        },
        "event_id": "$151800234427abgho:localhost",
        "membership": "join",
        "origin_server_ts": 151800152,
        "sender": "@example:localhost",
        "state_key": "@example:localhost",
        "type": "m.room.member",
        "prev_content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "join"
        },
        "unsigned": {
            "age": 297032,
            "replaces_state": "$151800140517rfvjc:localhost"
        }
    });
}

lazy_static! {
    pub static ref MEMBER_STRIPPED: JsonValue = json!({
        "content": {
            "avatar_url": null,
            "displayname": "example",
            "membership": "join"
        },
        "sender": "@example:localhost",
        "state_key": "@example:localhost",
        "type": "m.room.member",
    });
}

lazy_static! {
    pub static ref MESSAGE_EDIT: JsonValue = json!({
        "content": {
            "body": " * edited message",
            "m.new_content": {
                "body": "edited message",
                "msgtype": "m.text"
            },
            "m.relates_to": {
                "event_id": "$someeventid:foo",
                "rel_type": "m.replace"
            },
            "msgtype": "m.text"
        },
        "event_id": "$eventid:foo",
        "origin_server_ts": 159026265,
        "sender": "@alice:matrix.org",
        "type": "m.room.message",
        "unsigned": {
            "age": 85
        }
    });
}

lazy_static! {
    pub static ref MESSAGE_EMOTE: JsonValue = json!({
        "content": {
            "body": "is dancing", "format": "org.matrix.custom.html",
            "formatted_body": "<strong>is dancing</strong>",
            "msgtype": "m.emote"
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 152037280,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 598971
        }
    });
}

lazy_static! {
    pub static ref MESSAGE_NOTICE: JsonValue = json!({
      "origin_server_ts": 153356516,
      "sender": "@_neb_github:matrix.org",
      "event_id": "$153356516319138IHRIC:matrix.org",
      "unsigned": {
        "age": 743
      },
      "content": {
        "body": "https://github.com/matrix-org/matrix-python-sdk/issues/266 : Consider allowing MatrixClient.__init__ to take sync_token kwarg",
        "format": "org.matrix.custom.html",
        "formatted_body": "<a href='https://github.com/matrix-org/matrix-python-sdk/pull/313'>313: nio wins!</a>",
        "msgtype": "m.notice"
      },
      "type": "m.room.message",
      "room_id": "!YHhmBTmGBHGQOlGpaZ:matrix.org"
    });
}

lazy_static! {
    pub static ref MESSAGE_TEXT: JsonValue = json!({
        "content": {
            "body": "is dancing", "format": "org.matrix.custom.html",
            "formatted_body": "<strong>is dancing</strong>",
            "msgtype": "m.text"
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 152037280,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 598971
        }
    });
}

lazy_static! {
    pub static ref NAME: JsonValue = json!({
        "content": {
            "name": "room name"
        },
        "event_id": "$15139375513VdeRF:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.name",
        "unsigned": {
            "age": 703422
        }
    });
}

lazy_static! {
    pub static ref NAME_STRIPPED: JsonValue = json!({
        "content": {
            "name": "room name"
        },
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.name",
    });
}

lazy_static! {
    pub static ref POWER_LEVELS: JsonValue = json!({
        "content": {
            "ban": 50,
            "events": {
                "m.room.avatar": 50,
                "m.room.canonical_alias": 50,
                "m.room.history_visibility": 100,
                "m.room.name": 50,
                "m.room.power_levels": 100,
                "m.room.message": 25
            },
            "events_default": 0,
            "invite": 0,
            "kick": 50,
            "redact": 50,
            "state_default": 50,
            "users": {
                "@example:localhost": 100,
                "@bob:localhost": 0
            },
            "users_default": 0
        },
        "event_id": "$15139375512JaHAW:localhost",
        "origin_server_ts": 151393755,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.power_levels",
        "unsigned": {
            "age": 703422
        }
    }
    );
}

lazy_static! {
    pub static ref PRESENCE: JsonValue = json!({
        "content": {
            "avatar_url": "mxc://localhost/wefuiwegh8742w",
            "currently_active": false,
            "last_active_ago": 1,
            "presence": "online",
            "status_msg": "Making cupcakes"
        },
        "sender": "@example:localhost",
        "type": "m.presence"
    });
}

lazy_static! {
    pub static ref PUBLIC_ROOMS: JsonValue = json!({
        "chunk": [
            {
                "aliases": [
                    "#murrays:cheese.bar"
                ],
                "avatar_url": "mxc://bleeker.street/CHEDDARandBRIE",
                "guest_can_join": false,
                "name": "CHEESE",
                "num_joined_members": 37,
                "room_id": "!ol19s:bleecker.street",
                "topic": "Tasty tasty cheese",
                "world_readable": true
            }
        ],
        "next_batch": "p190q",
        "prev_batch": "p1902",
        "total_room_count_estimate": 115
    });
}

lazy_static! {
    pub static ref PUSH_RULES: JsonValue = json!({
        "content": {
            "global": {
                "content": [
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "pattern": "example",
                        "rule_id": ".m.rule.contains_user_name"
                    }
                ],
                "override": [
                    {
                        "actions": [
                            "dont_notify"
                        ],
                        "conditions": [],
                        "default": true,
                        "enabled": false,
                        "rule_id": ".m.rule.master"
                    },
                    {
                        "actions": [
                            "dont_notify"
                        ],
                        "conditions": [
                            {
                                "key": "content.msgtype",
                                "kind": "event_match",
                                "pattern": "m.notice"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.suppress_notices"
                    }
                ],
                "room": [],
                "sender": [],
                "underride": [
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "ring"
                            },
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.call.invite"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.call"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight"
                            }
                        ],
                        "conditions": [
                            {
                                "kind": "contains_display_name"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.contains_display_name"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "is": "2",
                                "kind": "room_member_count"
                            },
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.message"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.room_one_to_one"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "sound",
                                "value": "default"
                            },
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.member"
                            },
                            {
                                "key": "content.membership",
                                "kind": "event_match",
                                "pattern": "invite"
                            },
                            {
                                "key": "state_key",
                                "kind": "event_match",
                                "pattern": "@example:localhost"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.invite_for_me"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.member"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.member_event"
                    },
                    {
                        "actions": [
                            "notify",
                            {
                                "set_tweak": "highlight",
                                "value": false
                            }
                        ],
                        "conditions": [
                            {
                                "key": "type",
                                "kind": "event_match",
                                "pattern": "m.room.message"
                            }
                        ],
                        "default": true,
                        "enabled": true,
                        "rule_id": ".m.rule.message"
                    }
                ]
            }
        },
        "type": "m.push_rules"
    });
}

lazy_static! {
    pub static ref REGISTRATION_RESPONSE_ERR: JsonValue = json!({
        "errcode": "M_FORBIDDEN",
        "error": "Invalid password",
        "completed": ["example.type.foo"],
        "flows": [
            {
                "stages": ["example.type.foo", "example.type.bar"]
            },
            {
                "stages": ["example.type.foo", "example.type.baz"]
            }
        ],
        "params": {
            "example.type.baz": {
                "example_key": "foobar"
            }
        },
        "session": "xxxxxx"
    });
}

lazy_static! {
    pub static ref REACTION: JsonValue = json!({
        "content": {
            "m.relates_to": {
                "event_id": "$MDitXXXXXXuBlpP7S6c6XXXXXXXC2HqZ3peV1NrV4PKA",
                "key": "👍",
                "rel_type": "m.annotation"
            }
        },
        "event_id": "$QZn9xEXXXXXfd2tAGFH-XXgsffZlVMobk47Tl5Lpdtg",
        "origin_server_ts": 159027581,
        "sender": "@devinr528:matrix.org",
        "type": "m.reaction",
        "unsigned": {
            "age": 85
        }
    });
}

lazy_static! {
    pub static ref READ_RECEIPT: JsonValue = json!({
        "content": {
            "$example": {
                "m.read": {
                    "@example:localhost": {
                        "ts": 1436451550
                    }
                }
            }
        },
        "room_id": "!test:localhost",
        "type": "m.receipt"
    });
}

lazy_static! {
    pub static ref READ_RECEIPT_OTHER: JsonValue = json!({
        "content": {
            "$other": {
                "m.read": {
                    "@example:localhost": {
                        "ts": 1436964550
                    }
                }
            }
        },
        "room_id": "!test:localhost",
        "type": "m.receipt"
    });
}

lazy_static! {
    pub static ref REDACTED_INVALID: JsonValue = json!({
        "content": {},
        "event_id": "$15275046980maRLj:localhost",
        "origin_server_ts": 1527504698,
        "sender": "@example:localhost",
        "type": "m.room.message"
    });
}

lazy_static! {
    pub static ref REDACTED_STATE: JsonValue = json!({
        "content": {},
        "event_id": "$example_id:example.org",
        "origin_server_ts": 153232493,
        "sender": "@example:example.org",
        "state_key": "test_state_key",
        "type": "m.some.state",
        "unsigned": {
            "age": 3069315,
            "redacted_because": {
                "content": {},
                "event_id": "$redaction_example_id:example.org",
                "origin_server_ts": 153232494,
                "redacts": "$example_id:example.org",
                "sender": "@example:example:org",
                "type": "m.room.redaction",
                "unsigned": {"age": 30693147}
            },
            "redacted_by": "$redaction_example_id:example.org"
        }
    });
}

lazy_static! {
    pub static ref REDACTED: JsonValue = json!({
        "content": {},
        "event_id": "$15275046980maRLj:localhost",
        "origin_server_ts": 1527504698,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 19334,
            "redacted_because": {
                "content": {},
                "event_id": "$15275047031IXQRi:localhost",
                "origin_server_ts": 1527504703,
                "redacts": "$15275046980maRLj:localhost",
                "sender": "@example:localhost",
                "type": "m.room.redaction",
                "unsigned": {
                    "age": 14523
                }
            },
            "redacted_by": "$15275047031IXQRi:localhost"
        }
    });
}

lazy_static! {
    pub static ref REDACTION: JsonValue = json!({
        "content": {
            "reason": "😀"
        },
        "event_id": "$151957878228ssqrJ:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "type": "m.room.redaction",
        "redacts": "$151957878228ssqrj:localhost"
    });
}

lazy_static! {
    pub static ref ROOM_AVATAR: JsonValue = json!({
        "content": {
            "info": {
                "h": 398,
                "mimetype": "image/jpeg",
                "size": 31037,
                "w": 394
            },
            "url": "mxc://domain.com/JWEIFJgwEIhweiWJE"
        },
        "event_id": "$143273582443PhrSn:domain.com",
        "origin_server_ts": 143273582,
        "room_id": "!jEsUZKDJdhlrceRyVU:domain.com",
        "sender": "@example:domain.com",
        "state_key": "",
        "type": "m.room.avatar",
        "unsigned": {
            "age": 1234
        }
    });
}

lazy_static! {
    pub static ref ROOM_ID: JsonValue = json!({
        "room_id": "!testroom:example.org"
    });
}

lazy_static! {
    pub static ref TAG: JsonValue = json!({
        "content": {
            "tags": {
                "u.work": {
                    "order": 0.9
                }
            }
        },
        "type": "m.tag"
    });
}

// TODO: Move `prev_content` into `unsigned` once ruma supports it
lazy_static! {
    pub static ref TOPIC: JsonValue = json!({
        "content": {
            "topic": "😀"
        },
        "event_id": "$151957878228ssqrJ:localhost",
        "origin_server_ts": 151957878,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.topic",
        "prev_content": {
            "topic": "test"
        },
        "unsigned": {
          "age": 1392989,
          "prev_sender": "@example:localhost",
          "replaces_state": "$151957069225EVYKm:localhost"
        }
    });
}

lazy_static! {
    pub static ref TYPING: JsonValue = json!({
        "content": {
            "user_ids": [
                "@alice:matrix.org",
                "@bob:example.com"
            ]
        },
        "room_id": "!jEsUZKDJdhlrceRyVU:example.org",
        "type": "m.typing"
    });
}
