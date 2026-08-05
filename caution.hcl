enclave "default" {
  build {
    app_sources = ["git@codeberg.org:caution/demo-hello-world-enclave.git"]
    cache = false
  }

  network {
    ingress {
      cidr_ipv4  = "0.0.0.0/0"
      port       = 8083
      ip_protocol = "tcp"
    }

    egress {
      cidr_ipv4 = "0.0.0.0/0"
    }

    http {
      domain            = "chelupa.caution.dev"
      port              = 8083
      upstream_protocol = "h2c"

      e2e_encryption {
        mode = "tls"
      }
    }
  }

  unit "default" {
    command = "/usr/local/bin/grpc-hello-server"
  }
}
