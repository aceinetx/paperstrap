use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct ServerProperties {
    pub accepts_transfers: bool,
    pub allow_flight: bool,
    pub broadcast_console_to_ops: bool,
    pub broadcast_rcon_to_ops: bool,
    pub bug_report_link: String,
    pub chat_spam_threshold_seconds: u16,
    pub command_spam_threshold_seconds: u16,
    pub debug: bool,
    pub difficulty: String,
    pub enable_code_of_conduct: bool,
    pub enable_jmx_monitoring: bool,
    pub enable_query: bool,
    pub enable_rcon: bool,
    pub enable_status: bool,
    pub enforce_secure_profile: bool,
    pub enforce_whitelist: bool,
    pub entity_broadcast_range_percentage: usize,
    pub force_gamemode: bool,
    pub function_permission_level: usize,
    pub gamemode: String,
    pub generate_structures: bool,
    pub generator_settings: String,
    pub hardcore: bool,
    pub hide_online_players: bool,
    pub initial_disabled_packs: String,
    pub initial_enabled_packs: String,
    pub level_name: String,
    pub level_seed: String,
    pub level_type: String,
    pub log_ips: bool,
    pub management_server_allowed_origins: String,
    pub management_server_enabled: bool,
    pub management_server_host: String,
    pub management_server_port: u16,
    pub management_server_tls_enabled: bool,
    pub management_server_tls_keystore: String,
    pub management_server_tls_keystore_password: String,
    pub max_chained_neighbor_updates: usize,
    pub max_players: usize,
    pub max_tick_time: usize,
    pub max_world_size: usize,
    pub motd: String,
    pub network_compression_threshold: usize,
    pub online_mode: bool,
    pub op_permission_level: usize,
    pub pause_when_empty_seconds: i64,
    pub player_idle_timeout: usize,
    pub prevent_proxy_connections: bool,
    pub query_port: usize,
    pub rate_limit: usize,
    pub rcon_password: String,
    pub rcon_port: u16,
    pub region_file_compression: String,
    pub require_resource_pack: bool,
    pub resource_pack: String,
    pub resource_pack_id: String,
    pub resource_pack_prompt: String,
    pub resource_pack_sha1: String,
    pub server_ip: String,
    pub server_port: u16,
    pub simulation_distance: usize,
    pub spawn_protection: usize,
    pub status_heartbeat_interval: usize,
    pub sync_chunk_writes: bool,
    pub text_filtering_config: String,
    pub text_filtering_version: usize,
    pub use_native_transport: bool,
    pub view_distance: usize,
    pub white_list: bool,
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self {
            accepts_transfers: false,
            allow_flight: false,
            broadcast_console_to_ops: true,
            broadcast_rcon_to_ops: true,
            bug_report_link: "".into(),
            chat_spam_threshold_seconds: 10,
            command_spam_threshold_seconds: 10,
            debug: false,
            difficulty: "easy".into(),
            enable_code_of_conduct: false,
            enable_jmx_monitoring: false,
            enable_query: false,
            enable_rcon: false,
            enable_status: true,
            enforce_secure_profile: true,
            enforce_whitelist: false,
            entity_broadcast_range_percentage: 100,
            force_gamemode: false,
            function_permission_level: 2,
            gamemode: "survival".into(),
            generate_structures: true,
            generator_settings: "{}".into(),
            hardcore: false,
            hide_online_players: false,
            initial_disabled_packs: "".into(),
            initial_enabled_packs: "vanilla".into(),
            level_name: "world".into(),
            level_seed: "".into(),
            level_type: "minecraft:normal".into(),
            log_ips: true,
            management_server_allowed_origins: "".into(),
            management_server_enabled: false,
            management_server_host: "localhost".into(),
            management_server_port: 0,
            management_server_tls_enabled: true,
            management_server_tls_keystore: "".into(),
            management_server_tls_keystore_password: "".into(),
            max_chained_neighbor_updates: 1000000,
            max_players: 20,
            max_tick_time: 60000,
            max_world_size: 29999984,
            motd: "paperstrap worked! Add the motd property to the server_properties record to change this message".into(),
            network_compression_threshold: 256,
            online_mode: false,
            op_permission_level: 4,
            pause_when_empty_seconds: -1,
            player_idle_timeout: 0,
            prevent_proxy_connections: false,
            query_port: 25565,
            rate_limit: 0,
            rcon_password: "".into(),
            rcon_port: 25575,
            region_file_compression: "deflate".into(),
            require_resource_pack: false,
            resource_pack: "".into(),
            resource_pack_id: "".into(),
            resource_pack_prompt: "".into(),
            resource_pack_sha1: "".into(),
            server_ip: "".into(),
            server_port: 25565,
            simulation_distance: 10,
            spawn_protection: 0,
            status_heartbeat_interval: 0,
            sync_chunk_writes: true,
            text_filtering_config: "".into(),
            text_filtering_version: 0,
            use_native_transport: true,
            view_distance: 10,
            white_list: true,
        }
    }
}

impl Display for ServerProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"# generated by paperstrap, do not change
accepts-transfers={}
allow-flight={}
broadcast-console-to-ops={}
broadcast-rcon-to-ops={}
bug-report-link={}
chat-spam-threshold-seconds={}
command-spam-threshold-seconds={}
debug={}
difficulty={}
enable-code-of-conduct={}
enable-jmx-monitoring={}
enable-query={}
enable-rcon={}
enable-status={}
enforce-secure-profile={}
enforce-whitelist={}
entity-broadcast-range-percentage={}
force-gamemode={}
function-permission-level={}
gamemode={}
generate-structures={}
generator-settings={}
hardcore={}
hide-online-players={}
initial-disabled-packs={}
initial-enabled-packs={}
level-name={}
level-seed={}
level-type={}
log-ips={}
management-server-allowed-origins={}
management-server-enabled={}
management-server-host={}
management-server-port={}
management-server-tls-enabled={}
management-server-tls-keystore={}
management-server-tls-keystore-password={}
max-chained-neighbor-updates={}
max-players={}
max-tick-time={}
max-world-size={}
motd={}
network-compression-threshold={}
online-mode={}
op-permission-level={}
pause-when-empty-seconds={}
player-idle-timeout={}
prevent-proxy-connections={}
query.port={}
rate-limit={}
rcon.password={}
rcon.port={}
region-file-compression={}
require-resource-pack={}
resource-pack={}
resource-pack-id={}
resource-pack-prompt={}
resource-pack-sha1={}
server-ip={}
server-port={}
simulation-distance={}
spawn-protection={}
status-heartbeat-interval={}
sync-chunk-writes={}
text-filtering-config={}
text-filtering-version={}
use-native-transport={}
view-distance={}
white-list={}
"#,
            self.accepts_transfers,
            self.allow_flight,
            self.broadcast_console_to_ops,
            self.broadcast_rcon_to_ops,
            self.bug_report_link,
            self.chat_spam_threshold_seconds,
            self.command_spam_threshold_seconds,
            self.debug,
            self.difficulty,
            self.enable_code_of_conduct,
            self.enable_jmx_monitoring,
            self.enable_query,
            self.enable_rcon,
            self.enable_status,
            self.enforce_secure_profile,
            self.enforce_whitelist,
            self.entity_broadcast_range_percentage,
            self.force_gamemode,
            self.function_permission_level,
            self.gamemode,
            self.generate_structures,
            self.generator_settings,
            self.hardcore,
            self.hide_online_players,
            self.initial_disabled_packs,
            self.initial_enabled_packs,
            self.level_name,
            self.level_seed,
            self.level_type,
            self.log_ips,
            self.management_server_allowed_origins,
            self.management_server_enabled,
            self.management_server_host,
            self.management_server_port,
            self.management_server_tls_enabled,
            self.management_server_tls_keystore,
            self.management_server_tls_keystore_password,
            self.max_chained_neighbor_updates,
            self.max_players,
            self.max_tick_time,
            self.max_world_size,
            self.motd,
            self.network_compression_threshold,
            self.online_mode,
            self.op_permission_level,
            self.pause_when_empty_seconds,
            self.player_idle_timeout,
            self.prevent_proxy_connections,
            self.query_port,
            self.rate_limit,
            self.rcon_password,
            self.rcon_port,
            self.region_file_compression,
            self.require_resource_pack,
            self.resource_pack,
            self.resource_pack_id,
            self.resource_pack_prompt,
            self.resource_pack_sha1,
            self.server_ip,
            self.server_port,
            self.simulation_distance,
            self.spawn_protection,
            self.status_heartbeat_interval,
            self.sync_chunk_writes,
            self.text_filtering_config,
            self.text_filtering_version,
            self.use_native_transport,
            self.view_distance,
            self.white_list,
        )
    }
}
