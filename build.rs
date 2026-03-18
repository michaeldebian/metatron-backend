fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &[
        // Shared protos (also used by mcp-server, grpc-stub)
        "../proto/cluster.proto",
        "../proto/report.proto",
        "../proto/finops.proto",
        "../proto/mcp_gateway.proto",
        // Backend-only protos
        "proto/auth.proto",
        "proto/pipeline.proto",
        "proto/analytics.proto",
    ];
    let include_dirs = &["../proto", "proto"];

    tonic_build::configure()
        .build_server(true) // serves AuthService, PipelineService, AnalyticsService
        .build_client(true) // calls ClusterService, FinOpsService, McpGatewayService
        .compile(proto_files, include_dirs)?;

    for proto in proto_files {
        println!("cargo:rerun-if-changed={proto}");
    }

    Ok(())
}
