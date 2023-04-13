use std::path::Path;
use std::sync::Arc;

use crate::model::CompareResults;
use warp::Filter;

pub async fn start_server(image_paths: Vec<String>, compare_results: Arc<CompareResults>) {
    let images_dir = tempdir::TempDir::new("audio-compare").expect("Failed to create temp dir");
    std::fs::create_dir_all(format!("{}/images", images_dir.path().display()))
        .expect("Failed to create images dir");
    image_paths.iter().for_each(|image| {
        let image_name = Path::new(image).file_name().unwrap().to_str().unwrap();
        let target = format!("{}/images/{}", images_dir.path().display(), image_name);
        log::info!("Copying image {} to {}", image, target);
        std::fs::copy(image, target).expect("Failed to move image file");
    });

    let images_dir_route = warp::path("images")
        .and(warp::fs::dir(images_dir.path().join("images")))
        .with(warp::log("images"));
    let home_route = warp::get()
        .and(warp::path::end())
        .map(move || handle_get_home(&compare_results))
        .with(warp::log("home"));
    warp::serve(home_route.or(images_dir_route))
        .run(([127, 0, 0, 1], 3030))
        .await
}

fn handle_get_home(results: &CompareResults) -> warp::reply::Html<String> {
    let mut context = tera::Context::new();
    context.insert("metadatas", &results.metadatas);
    context.insert("similarities", &results.similarities);
    let result = tera::Tera::one_off(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Audio Compare</title>
        <style>

body {
  font-family: sans-serif;
}        

table {
  border-spacing: 0;
  width: 100%;
}

table th,
table td {
  text-align: left;
  border: 1px solid rgba(0,0,0,0.3);
  padding: 2px 4px;
  box-sizing: border-box;
}

table td img {
  max-width: 200px;
  max-height: 200px;
}
        </style>
    </head>
    <body>
        <h1>Audio Compare</h1>
        <h2>Metadata</h2>
        <table>
             <thead>
                 <tr>
                      <th>File</th>
                      <th>Duration seconds</th>
                      <th>Sample rate</th>
                      <th>Channels</th>
                      <th>Bits per sample</th>
                      <th>Image</th>
                 </tr>
                </thead>
                <tbody>
                 {% for metadata in metadatas %}
                      <tr>
                            <td>{{ metadata.path }}</td>
                            <td>{{ metadata.duration_seconds }}</td>
                            <td>{{ metadata.spec.sample_rate }}Hz</td>
                            <td>{{ metadata.spec.channels }}</td>
                            <td>{{ metadata.spec.bits_per_sample }}</td>
                            <td>
                                <img src="/images/{{ metadata.filename }}--audio.png" />
                            </td>
                      </tr>
                 {% endfor %}
             </tbody>
        </table>

        <h2>Cross-correlation similarity</h2>
        <table>
            <thead>
                <tr>
                    <th>File 1</th>
                    <th>File 2</th>
                    <th>Cross-correlation Similarity</th>
                    <th>Spectral similarity</th>
                    <th>Delta magnitude</th>
                </tr>
            </thead>
            <tbody>
                {% for similarity in similarities %}
                    <tr>
                        <td>{{ similarity.file1 }}</td>
                        <td>{{ similarity.file2 }}</td>
                        <td>{{ similarity.cross_correlation_similarity | round(method="floor", precision=4) }}</td>
                        <td>{{ similarity.spectral_similarity | round(method="floor", precision=4) }}</td>
                        <td>{{ similarity.delta_magnitude | round(method="floor", precision=4) }}</td>
                    </tr>
                {% endfor %}
            </tbody>
        </table>
    </body>
</html>        
        "#,
        &mut context,
        false,
    );

    warp::reply::html(result.unwrap_or_else(|err| {
        log::error!("Failed to compile template: {}", err);
        format!("Failed to compile template: {}", err)
    }))
}
