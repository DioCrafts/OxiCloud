{
  config,
  lib,
  pkgs,
  oxicloud ? callPackage ./package.nix {},
  ...
}: let
  cfg = config.services.oxicloud;

  generatedEnv = pkgs.writeText "oxicloud-generated.env" ''
    OXICLOUD_STORAGE_PATH=${cfg.storagePath}
    OXICLOUD_STATIC_PATH=${cfg.staticPath}
    OXICLOUD_SERVER_PORT=${builtins.toString cfg.port}
    OXICLOUD_SERVER_HOST=${cfg.host}
    OXICLOUD_BASE_URL=${cfg.baseUrl}

    OXICLOUD_DB_CONNECTION_STRING=postgres://${cfg.database.user}:${builtins.readFile cfg.database.password.file}@${cfg.database.host}:${builtins.toString cfg.database.port}/${cfg.database.name}
    OXICLOUD_DB_MAX_CONNECTIONS=${builtins.toString cfg.database.maxConnections}
    OXICLOUD_DB_MIN_CONNECTIONS=${builtins.toString cfg.database.minConnections}

    OXICLOUD_ENABLE_AUTH=${lib.boolToString cfg.auth.enable}
    OXICLOUD_JWT_SECRET=${builtins.readFile cfg.auth.jwtSecret.file}

    OXICLOUD_ACCESS_TOKEN_EXPIRY_SECS=${builtins.toString cfg.auth.accessTokenExpirySecs}
    OXICLOUD_REFRESH_TOKEN_EXPIRY_SECS=${builtins.toString cfg.auth.refreshTokenExpirySecs}

    OXICLOUD_ENABLE_USER_STORAGE_QUOTAS=${lib.boolToString cfg.features.userStorageQuotas}
    OXICLOUD_ENABLE_FILE_SHARING=${lib.boolToString cfg.features.fileSharing}
    OXICLOUD_ENABLE_TRASH=${lib.boolToString cfg.features.trash}
    OXICLOUD_ENABLE_SEARCH=${lib.boolToString cfg.features.search}

    OXICLOUD_OIDC_ENABLED=${lib.boolToString cfg.oauth2.enable}
    OXICLOUD_OIDC_ISSUER_URL=${cfg.oauth2.issuerUrl}
    OXICLOUD_OIDC_CLIENT_ID=${cfg.oauth2.clientId}
    OXICLOUD_OIDC_CLIENT_SECRET=${builtins.readFile cfg.oauth2.clientSecret.file}
    OXICLOUD_OIDC_REDIRECT_URI=${cfg.oauth2.redirectUri}
    OXICLOUD_OIDC_SCOPES=${cfg.oauth2.scopes}
    OXICLOUD_OIDC_FRONTEND_URL=${cfg.oauth2.frontendUrl}

    OXICLOUD_WOPI_ENABLED=${lib.boolToString cfg.wopi.enable}
    OXICLOUD_WOPI_DISCOVERY_URL=${cfg.wopi.discoveryUrl}
    OXICLOUD_WOPI_SECRET=${builtins.readFile cfg.wopi.secret.file}
  '';
in {
  options.services.oxicloud = {
    enable = lib.mkEnableOption "OxiCloud";

    envFile = lib.mkOption {
      type = lib.types.path;
      default = "/etc/oxicloud/oxicloud.env";
      description = "Environment file containing secrets (DATABASE_URL, etc.)";
    };

    dataDir = lib.mkOption {
      type = lib.types.path;
      default = "/var/lib/oxicloud";
      description = "Directory for OxiCloud data";
    };

    storagePath = lib.mkOption {
      type = lib.types.path;
      default = "${cfg.dataDir}/storage";
      description = "Path for storage file storage";
    };

    staticPath = lib.mkOption {
      type = lib.types.path;
      default = "${cfg.dataDir}/static";
      description = "Path for static file storage";
    };

    port = lib.mkOption {
      type = lib.types.int;
      default = 8086;
      description = "Server port";
    };

    host = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "Server bind address";
    };

    baseUrl = lib.mkOption {
      type = lib.types.str;
      default = "http://${cfg.host}:${builtins.toString cfg.port}";
      description = "Public base URL for share links";
    };

    database = {
      user = lib.mkOption {
        type = lib.types.str;
        default = "oxicloud";
        description = "Name of the Postgresql database (if not database url)";
      };

      password.file = lib.mkOption {
        type = lib.types.str;
        default = "oxicloud";
        description = "Path to the database password (if not database url)";
      };

      host = lib.mkOption {
        type = lib.types.str;
        default = "localhost";
        description = "Host to the postgresql database (if not database url)";
      };

      port = lib.mkOption {
        type = lib.types.int;
        default = 5432;
        description = "Port to the postgresql database (if not database url)";
      };

      maxConnections = lib.mkOption {
        type = lib.types.int;
        default = 20;
        description = "Max pool connections";
      };

      minConnections = lib.mkOption {
        type = lib.types.int;
        default = 5;
        description = "Min pool connections";
      };
    };

    auth = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Enable authentication";
      };

      jwtSecret.file = lib.mkOption {
        type = lib.types.str;
        default = "/etc/oxicloud/jwt.secret";
        description = "JWT signing secret";
      };

      accessTokenExpirySecs = lib.mkOption {
        type = lib.types.int;
        default = 3600;
        description = "Access token lifetime (seconds)";
      };

      refreshTokenExpirySecs = lib.mkOption {
        type = lib.types.int;
        default = 2592000;
        description = "Refresh token lifetime (seconds)";
      };
    };

    features = {
      userStorageQuotas = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Per-user storage quotas";
      };

      fileSharing = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "File/folder sharing";
      };

      trash = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Trash / recycle bin";
      };

      search = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Search";
      };
    };

    oauth2 = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Enable OIDC";
      };

      issuerUrl = lib.mkOption {
        type = lib.types.str;
        default = "";
        description = "OIDC issuer URL";
      };

      clientId = lib.mkOption {
        type = lib.types.str;
        default = "";
        description = "Client ID";
      };

      clientSecret.file = lib.mkOption {
        type = lib.types.str;
        default = "/etc/oxicloud/oidc.secret";
        description = "Client secret";
      };

      redirectUri = lib.mkOption {
        type = lib.types.str;
        default = "http://localhost:8086/api/auth/oidc/callback";
        description = "Callback URL";
      };

      scopes = lib.mkOption {
        type = lib.types.str;
        default = "openid";
        description = "profile email	Requested scopes";
      };

      frontendUrl = lib.mkOption {
        type = lib.types.str;
        default = "http://${cfg.host}:${builtins.toString cfg.port}";
        description = "Frontend URL";
      };

      autoProvision = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Auto-create users on first SSO login";
      };

      adminGroups = lib.mkOption {
        type = lib.types.str;
        default = "";
        description = "Groups that grant admin role";
      };

      disablePasswordLogin = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Hide password form when OIDC enabled";
      };

      providerName = lib.mkOption {
        type = lib.types.str;
        default = "SSO";
        description = "Display name for the provider";
      };
    };
    wopi = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Enable WOPI";
      };

      discoveryUrl = lib.mkOption {
        type = lib.types.str;
        default = "";
        description = "Collabora/OnlyOffice discovery URL";
      };

      secret.file = lib.mkOption {
        type = lib.types.str;
        default = "/etc/oxicloud/wopi.secret";
        description = "(JWT secret) WOPI token signing key";
      };

      tokenTtlSecs = lib.mkOption {
        type = lib.types.int;
        default = 86400;
        description = "Token lifetime";
      };

      lockTtlSecs = lib.mkOption {
        type = lib.types.int;
        default = 1800;
        description = "Lock expiration";
      };
    };

    package = lib.mkOption {
      type = lib.types.package;
      default = oxicloud;
      description = "oxicloud nix package";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "oxicloud";
      description = "User to run OxiCloud service";
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "services";
      description = "Group to run OxiCloud service";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.oxicloud-env = {
      description = "Prepare OxiCloud environment file";

      wantedBy = ["multi-user.target"];
      before = ["oxicloud.service"];

      serviceConfig.Type = "oneshot";

      script = ''
        install -d -m 0755 ${cfg.dataDir}

        cp ${generatedEnv} ${cfg.dataDir}/.env

        if [ -f ${cfg.envFile} ]; then
          # append overrides (last wins in dotenv parsing)
          cat ${cfg.envFile} >> ${cfg.dataDir}/.env
        fi

        chown ${cfg.user}:${cfg.group} ${cfg.dataDir}/.env
        chmod 640 ${cfg.dataDir}/.env
      '';
    };

    systemd.tmpfiles.rules = [
      "d ${cfg.dataDir} 0700 ${cfg.user} ${cfg.group} -"
      "d ${cfg.storagePath} 0700 ${cfg.user} ${cfg.group} -"
      "d ${cfg.staticPath} 0750 ${cfg.user} ${cfg.group} -"
      "d /etc/oxicloud 0750 root root -"
      "f ${cfg.auth.jwtSecret.file} 0600 oxicloud services -"
      "f ${cfg.envFile} 0600 oxicloud services -"
      "f ${cfg.oauth2.clientSecret.file} 0600 oxicloud services -"
      "f ${cfg.wopi.secret.file} 0600 oxicloud services -"
    ];

    # Create user/group only if using defaults
    users.users = lib.mkIf (cfg.user == "oxicloud") {
      oxicloud = {
        isSystemUser = true;
        group = cfg.group;
        description = "OxiCloud service user";
      };
    };

    users.groups = lib.mkIf (cfg.group == "oxicloud") {
      oxicloud = {};
    };

    systemd.services.oxicloud = {
      description = "OxiCloud server";
      after = ["network.target" "postgresql.service" "oxicloud-env.service"];
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        User = cfg.user;
        Group = cfg.group;
        EnvironmentFile = [
          "-${cfg.dataDir}/.env"
          "-${generatedEnv}"
          "-${cfg.envFile}"
        ];
        ExecStart = "${cfg.package}/bin/oxicloud";
        Restart = "always";
        WorkingDirectory = cfg.dataDir;
        ReadWritePaths = [cfg.dataDir];

        # 🔒 hardening
        # ProtectSystem = "strict";
        # ProtectHome = true;

        AmbientCapabilities = lib.mkIf (cfg.port < 1024) ["CAP_NET_BIND_SERVICE"];
        CapabilityBoundingSet =
          if (cfg.port < 1024)
          then ["CAP_NET_BIND_SERVICE"]
          else [""];
        DeviceAllow = [""];
        LockPersonality = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectProc = "invisible";
        ProtectSystem = "full";
        RemoveIPC = true;
        RestrictAddressFamilies = [
          "AF_INET"
          "AF_INET6"
          "AF_UNIX"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
      };
    };
  };
}
