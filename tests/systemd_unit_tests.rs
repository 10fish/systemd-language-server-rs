use systemd_language_server::parse_unit_file;

#[test]
fn test_service_unit_file() {
    let content = r#"[Unit]
Description=My Test Service
Documentation=https://example.com/docs
After=network.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/bin/my-service --config=/etc/my-service/config.conf
ExecStop=/usr/bin/my-service --stop
Restart=on-failure
RestartSec=5
User=nobody
Group=nogroup
WorkingDirectory=/var/lib/my-service

[Install]
WantedBy=multi-user.target
"#;

    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());

    let ini = parsed.unwrap();

    // Check Unit section
    assert_eq!(ini.get("Unit", "Description").unwrap(), "My Test Service");
    assert_eq!(
        ini.get("Unit", "Documentation").unwrap(),
        "https://example.com/docs"
    );
    assert_eq!(ini.get("Unit", "After").unwrap(), "network.target");
    assert_eq!(ini.get("Unit", "Wants").unwrap(), "network-online.target");

    // Check Service section
    assert_eq!(ini.get("Service", "Type").unwrap(), "simple");
    assert_eq!(
        ini.get("Service", "ExecStart").unwrap(),
        "/usr/bin/my-service --config=/etc/my-service/config.conf"
    );
    assert_eq!(
        ini.get("Service", "ExecStop").unwrap(),
        "/usr/bin/my-service --stop"
    );
    assert_eq!(ini.get("Service", "Restart").unwrap(), "on-failure");
    assert_eq!(ini.get("Service", "RestartSec").unwrap(), "5");
    assert_eq!(ini.get("Service", "User").unwrap(), "nobody");
    assert_eq!(ini.get("Service", "Group").unwrap(), "nogroup");
    assert_eq!(
        ini.get("Service", "WorkingDirectory").unwrap(),
        "/var/lib/my-service"
    );

    // Check Install section
    assert_eq!(ini.get("Install", "WantedBy").unwrap(), "multi-user.target");
}

#[test]
fn test_socket_unit_file() {
    let content = r#"[Unit]
Description=Socket for My Service
Documentation=https://example.com/docs

[Socket]
ListenStream=8080
Accept=yes
SocketUser=nobody
SocketGroup=nogroup

[Install]
WantedBy=sockets.target
"#;

    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());

    let ini = parsed.unwrap();

    // Check Unit section
    assert_eq!(
        ini.get("Unit", "Description").unwrap(),
        "Socket for My Service"
    );

    // Check Socket section
    assert_eq!(ini.get("Socket", "ListenStream").unwrap(), "8080");
    assert_eq!(ini.get("Socket", "Accept").unwrap(), "yes");
    assert_eq!(ini.get("Socket", "SocketUser").unwrap(), "nobody");
    assert_eq!(ini.get("Socket", "SocketGroup").unwrap(), "nogroup");

    // Check Install section
    assert_eq!(ini.get("Install", "WantedBy").unwrap(), "sockets.target");
}

#[test]
fn test_timer_unit_file() {
    let content = r#"[Unit]
Description=Timer for My Service
Documentation=https://example.com/docs

[Timer]
OnBootSec=10min
OnUnitActiveSec=1h
OnCalendar=*-*-* 00:00:00
Unit=my-service.service

[Install]
WantedBy=timers.target
"#;

    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());

    let ini = parsed.unwrap();

    // Check Unit section
    assert_eq!(
        ini.get("Unit", "Description").unwrap(),
        "Timer for My Service"
    );

    // Check Timer section
    assert_eq!(ini.get("Timer", "OnBootSec").unwrap(), "10min");
    assert_eq!(ini.get("Timer", "OnUnitActiveSec").unwrap(), "1h");
    assert_eq!(ini.get("Timer", "OnCalendar").unwrap(), "*-*-* 00:00:00");
    assert_eq!(ini.get("Timer", "Unit").unwrap(), "my-service.service");

    // Check Install section
    assert_eq!(ini.get("Install", "WantedBy").unwrap(), "timers.target");
}

#[test]
fn test_mount_unit_file() {
    let content = r#"[Unit]
Description=Mount for /data
Documentation=https://example.com/docs

[Mount]
What=/dev/sda1
Where=/data
Type=ext4
Options=defaults

[Install]
WantedBy=multi-user.target
"#;

    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());

    let ini = parsed.unwrap();

    // Check Unit section
    assert_eq!(ini.get("Unit", "Description").unwrap(), "Mount for /data");

    // Check Mount section
    assert_eq!(ini.get("Mount", "What").unwrap(), "/dev/sda1");
    assert_eq!(ini.get("Mount", "Where").unwrap(), "/data");
    assert_eq!(ini.get("Mount", "Type").unwrap(), "ext4");
    assert_eq!(ini.get("Mount", "Options").unwrap(), "defaults");

    // Check Install section
    assert_eq!(ini.get("Install", "WantedBy").unwrap(), "multi-user.target");
}

#[test]
fn test_invalid_section_format() {
    let content = r#"[Unit
Description=Invalid Section Format
"#;

    let parsed = parse_unit_file(content);
    assert!(parsed.is_err());
}

#[test]
fn test_empty_file() {
    let content = "";

    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());

    let ini = parsed.unwrap();
    assert!(ini.get("Unit", "Description").is_none());
}

#[test]
fn test_comments() {
    let content = r#"# This is a comment
[Unit]
Description=Service with comments
# This is another comment
After=network.target # This is an inline comment
"#;

    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());

    let ini = parsed.unwrap();
    assert_eq!(
        ini.get("Unit", "Description").unwrap(),
        "Service with comments"
    );
    assert!(ini.get("Unit", "After").unwrap().contains("network.target"));
}

#[test]
fn test_parse_valid_unit_file() {
    let content = "[Unit]\nDescription=Test Service\n";
    let parsed = parse_unit_file(content);
    assert!(parsed.is_ok());
    let ini = parsed.unwrap();
    assert_eq!(ini.get("Unit", "Description").unwrap(), "Test Service");
}

#[test]
fn test_parse_invalid_unit_file() {
    let content = "[Unit\nDescription=Test Service\n";
    let parsed = parse_unit_file(content);
    assert!(parsed.is_err());
}
