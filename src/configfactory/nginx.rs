pub fn get_config_https(host : &str, server : &str, cache_max_size : &str, cache_ttl : &str) -> String {
    let host_underscore = host.replace(".", "_");
    let config = format!(
        r#"proxy_cache_path /var/www/cache/{host} levels=1:2 keys_zone={host_underscore}:10m max_size={cache_max_size} inactive={cache_ttl} use_temp_path=off;

server {{
    server_name {host};

    location / {{
        proxy_pass https://{server}.edge.icbix.com;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Enable caching
        proxy_cache ban_gy;
        proxy_cache_valid 200 302 {cache_ttl};
        proxy_cache_valid 404 1m;
        add_header X-Proxy-Cache $upstream_cache_status;
    }}

    listen 443 ssl;
    ssl_certificate /var/www/ssl/{host}/fullchain.pem;
    ssl_certificate_key /var/www/ssl/{host}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot

}}

server {{
    if ($host = {host}) {{
        return 301 https://$host$request_uri;
    }}

    listen 80;
    server_name {host};
    return 404;

}}
"#,
        host = host, 
        server = server
    );
    return config;
}