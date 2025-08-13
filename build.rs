extern crate tonic_build;

use std::fs;

fn main() {
    let mut protos: Vec<String> = Vec::new();

    let service_entries = fs::read_dir("./src/protos/aruna/api/storage/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/storage/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/aruna/api/notification/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/notification/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/aruna/api/dataproxy/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/dataproxy/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/aruna/api/hooks/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/hooks/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/aruna/api/health/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/health/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::prost_wkt_types")
        .build_server(true)
        .out_dir("./src/aruna")
        .compile_protos(
            &protos,
            &[
                "./src/protos".to_string(),
                "./src/protos/aruna/api/google".to_string(),
                "./src/protos/aruna/api/protoc-gen-openapiv2".to_string(),
            ],
        )
        .unwrap();

    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::prost_wkt_types")
        .build_server(true)
        .build_transport(false)
        .out_dir("./src/aruna_no_transport")
        .compile_protos(
            &protos,
            &[
                "./src/protos".to_string(),
                "./src/protos/aruna/api/google".to_string(),
                "./src/protos/aruna/api/protoc-gen-openapiv2".to_string(),
            ],
        )
        .unwrap();
}
