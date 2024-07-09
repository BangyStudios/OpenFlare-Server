# OpenFlare

## Introduction

OpenFlare is an open-source self-hosted alternative to Cloudflare, designed to provide content delivery and web acceleration. Utilizing NGINX for reverse caching, OpenCDN offers a comprehensive solution with automatic configuration generation, health checks, SSL auto-deployment, software load balancing, DNS GeoIP routing, all packaged within a Docker container. This solution can be deployed on VPS servers or even on Raspberry Pis hosted in your local coffee shop.

## Features (Planned)

- **Configuration Autogeneration:** Automatically generate NGINX/DNS configurations for multi-node deployments.
- **Health Checks:** Consensus mechanism for failovers to maximize uptime of websites hosted.
- **SSL Autodeployment:** Automatically deploy and renew SSL certificates using certbot.
- **Software Load Balancing:** Distribute traffic evenly across multiple nodes based on traffic and server capacity.
- **DNS GeoIP CDN:** Direct users to the nearest server location for faster response times based on their geographic location.
- **Docker Container Deployment:** Easily deploy and manage your nodes using a Docker container.
- **WebSocket Reverse Tunneling:** Allows nodes behind firewalls to tunnel to the browser without the need for port-forwarding.
- **Frontend:** To manage your nodes via an intuitive graphical user interface.

## Contributing

We welcome contributions from the community! If you'd like to contribute, please fork the repository and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, please open an issue on GitHub.

---

Made with ❤️ by Bangy ☕