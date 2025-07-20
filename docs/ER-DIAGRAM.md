erDiagram
    %% Core Tenant Management
    TENANT {
        uuid id PK
        string name
        string slug "unique"
        string domain "optional custom domain"
        string logo_url
        string primary_color
        string secondary_color
        json branding_config
        json settings "tenant-specific settings"
        enum status "active, suspended, deleted"
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    %% User Management
    USER {
        uuid id PK
        uuid tenant_id FK
        string email "unique within tenant"
        string username "unique within tenant, optional"
        string password_hash
        string first_name
        string last_name
        string phone
        string avatar_url
        json custom_attributes
        boolean email_verified
        boolean phone_verified
        boolean is_active
        boolean force_password_change
        timestamp password_changed_at
        timestamp last_login_at
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    %% Authentication & Sessions
    USER_SESSION {
        uuid id PK
        uuid user_id FK
        string session_token "unique"
        string device_info
        string ip_address
        string user_agent
        timestamp expires_at
        timestamp created_at
        timestamp last_accessed_at
    }

    MFA_DEVICE {
        uuid id PK
        uuid user_id FK
        enum type "totp, sms, email"
        string device_name
        string secret "encrypted"
        json backup_codes "encrypted array"
        boolean is_verified
        boolean is_primary
        timestamp created_at
        timestamp last_used_at
    }

    PASSWORD_RESET {
        uuid id PK
        uuid user_id FK
        string token "unique"
        timestamp expires_at
        boolean used
        timestamp created_at
    }

    %% Role-Based Access Control
    ROLE {
        uuid id PK
        uuid tenant_id FK
        string name
        string description
        enum type "system, custom"
        json permissions "array of permission objects"
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    USER_ROLE {
        uuid id PK
        uuid user_id FK
        uuid role_id FK
        uuid assigned_by FK "references USER"
        timestamp assigned_at
        timestamp expires_at "optional"
        boolean is_active
    }

    PERMISSION {
        uuid id PK
        string name "unique"
        string resource
        string action
        string description
        enum category "user, admin, api, etc"
        timestamp created_at
    }

    ROLE_PERMISSION {
        uuid id PK
        uuid role_id FK
        uuid permission_id FK
        json conditions "optional conditions"
        timestamp created_at
    }

    %% Organization Structure
    ORGANIZATION_UNIT {
        uuid id PK
        uuid tenant_id FK
        uuid parent_id FK "self reference"
        string name
        string description
        string path "materialized path"
        integer level
        timestamp created_at
        timestamp updated_at
    }

    USER_GROUP {
        uuid id PK
        uuid tenant_id FK
        uuid organization_unit_id FK "optional"
        string name
        string description
        enum type "manual, automatic"
        json auto_assignment_rules "conditions for automatic assignment"
        timestamp created_at
        timestamp updated_at
    }

    USER_GROUP_MEMBERSHIP {
        uuid id PK
        uuid user_id FK
        uuid user_group_id FK
        uuid assigned_by FK "references USER, optional"
        enum assignment_type "manual, automatic"
        timestamp assigned_at
        timestamp expires_at "optional"
    }

    %% Identity Federation & SSO
    IDENTITY_PROVIDER {
        uuid id PK
        uuid tenant_id FK
        string name
        enum type "saml, oidc, social, ldap"
        json configuration "provider-specific config"
        json attribute_mapping
        boolean auto_provision
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    FEDERATED_IDENTITY {
        uuid id PK
        uuid user_id FK
        uuid identity_provider_id FK
        string external_id "user ID in external system"
        json attributes "attributes from external system"
        timestamp last_sync_at
        timestamp created_at
        timestamp updated_at
    }

    %% API & Service Accounts
    API_KEY {
        uuid id PK
        uuid user_id FK "optional, for user keys"
        uuid tenant_id FK
        string name
        string key_hash
        string key_prefix "visible part for identification"
        json scopes "array of permitted scopes"
        timestamp expires_at "optional"
        timestamp last_used_at
        boolean is_active
        timestamp created_at
    }

    SERVICE_ACCOUNT {
        uuid id PK
        uuid tenant_id FK
        string name
        string description
        json permissions
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    %% Audit & Compliance
    AUDIT_LOG {
        uuid id PK
        uuid tenant_id FK
        uuid user_id FK "optional"
        string event_type
        string resource_type
        uuid resource_id "optional"
        json event_data
        string ip_address
        string user_agent
        enum result "success, failure"
        timestamp created_at
    }

    LOGIN_ATTEMPT {
        uuid id PK
        uuid tenant_id FK
        uuid user_id FK "optional, null for failed attempts"
        string email_attempted
        string ip_address
        string user_agent
        enum result "success, failure"
        string failure_reason "optional"
        timestamp attempted_at
    }

    %% Access Governance
    ACCESS_REVIEW {
        uuid id PK
        uuid tenant_id FK
        string name
        string description
        enum type "user_access, role_certification, periodic"
        enum status "draft, active, completed, cancelled"
        uuid created_by FK "references USER"
        timestamp start_date
        timestamp end_date
        json configuration
        timestamp created_at
        timestamp updated_at
    }

    ACCESS_REVIEW_ITEM {
        uuid id PK
        uuid access_review_id FK
        uuid user_id FK
        uuid role_id FK "optional"
        uuid permission_id FK "optional"
        uuid reviewer_id FK "references USER"
        enum status "pending, approved, rejected, skipped"
        string justification "optional"
        timestamp reviewed_at
        timestamp created_at
    }

    ACCESS_REQUEST {
        uuid id PK
        uuid tenant_id FK
        uuid requester_id FK "references USER"
        enum request_type "role, permission, resource_access"
        uuid role_id FK "optional"
        json requested_permissions
        string justification
        enum status "pending, approved, rejected, cancelled"
        uuid approved_by FK "references USER, optional"
        timestamp expires_at "optional, for temporary access"
        timestamp created_at
        timestamp updated_at
    }

    %% Notifications & Communications
    NOTIFICATION_TEMPLATE {
        uuid id PK
        uuid tenant_id FK "optional, null for system templates"
        string name
        enum type "email, sms, in_app"
        string subject
        text template_body
        json variables
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    USER_NOTIFICATION {
        uuid id PK
        uuid user_id FK
        uuid notification_template_id FK
        string subject
        text content
        enum type "email, sms, in_app"
        enum status "pending, sent, delivered, failed"
        json metadata
        timestamp sent_at
        timestamp read_at
        timestamp created_at
    }

    %% Webhooks & Integrations
    WEBHOOK {
        uuid id PK
        uuid tenant_id FK
        string name
        string url
        json headers
        string secret
        json event_types "array of events to listen for"
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }

    WEBHOOK_DELIVERY {
        uuid id PK
        uuid webhook_id FK
        string event_type
        json payload
        integer response_status
        text response_body
        integer attempt_count
        timestamp delivered_at
        timestamp created_at
    }

    %% Configuration & Settings
    TENANT_SETTING {
        uuid id PK
        uuid tenant_id FK
        string setting_key
        json setting_value
        string description
        enum category "auth, security, ui, integration"
        timestamp updated_at
        timestamp created_at
    }

    FEATURE_FLAG {
        uuid id PK
        uuid tenant_id FK "optional, null for global flags"
        string flag_name
        boolean is_enabled
        json configuration "additional flag configuration"
        timestamp created_at
        timestamp updated_at
    }

    %% Relationships
    TENANT ||--o{ USER : "has many users"
    TENANT ||--o{ ROLE : "has many roles"
    TENANT ||--o{ USER_GROUP : "has many groups"
    TENANT ||--o{ ORGANIZATION_UNIT : "has many org units"
    TENANT ||--o{ IDENTITY_PROVIDER : "has many providers"
    TENANT ||--o{ API_KEY : "has many api keys"
    TENANT ||--o{ SERVICE_ACCOUNT : "has many service accounts"
    TENANT ||--o{ AUDIT_LOG : "has many audit logs"
    TENANT ||--o{ ACCESS_REVIEW : "has many access reviews"
    TENANT ||--o{ ACCESS_REQUEST : "has many access requests"
    TENANT ||--o{ NOTIFICATION_TEMPLATE : "has many templates"
    TENANT ||--o{ WEBHOOK : "has many webhooks"
    TENANT ||--o{ TENANT_SETTING : "has many settings"
    TENANT ||--o{ FEATURE_FLAG : "has many feature flags"

    USER ||--o{ USER_SESSION : "has many sessions"
    USER ||--o{ MFA_DEVICE : "has many mfa devices"
    USER ||--o{ PASSWORD_RESET : "has many reset requests"
    USER ||--o{ USER_ROLE : "has many role assignments"
    USER ||--o{ USER_GROUP_MEMBERSHIP : "belongs to many groups"
    USER ||--o{ FEDERATED_IDENTITY : "has many federated identities"
    USER ||--o{ API_KEY : "has many api keys"
    USER ||--o{ LOGIN_ATTEMPT : "has many login attempts"
    USER ||--o{ ACCESS_REQUEST : "has many access requests"
    USER ||--o{ USER_NOTIFICATION : "has many notifications"

    ROLE ||--o{ USER_ROLE : "assigned to many users"
    ROLE ||--o{ ROLE_PERMISSION : "has many permissions"
    ROLE ||--o{ ACCESS_REVIEW_ITEM : "reviewed in access reviews"

    PERMISSION ||--o{ ROLE_PERMISSION : "assigned to many roles"

    USER_GROUP ||--o{ USER_GROUP_MEMBERSHIP : "has many members"

    ORGANIZATION_UNIT ||--o{ ORGANIZATION_UNIT : "has child units"
    ORGANIZATION_UNIT ||--o{ USER_GROUP : "contains groups"

    IDENTITY_PROVIDER ||--o{ FEDERATED_IDENTITY : "provides identities"

    ACCESS_REVIEW ||--o{ ACCESS_REVIEW_ITEM : "has many items"

    NOTIFICATION_TEMPLATE ||--o{ USER_NOTIFICATION : "generates notifications"

    WEBHOOK ||--o{ WEBHOOK_DELIVERY : "has many deliveries"
