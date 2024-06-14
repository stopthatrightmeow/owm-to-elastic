# Open Weather Map to Elasticsearch

## Introduction

To bridge the gap between theoretical knowledge and practical application, we often need to engage in hands-on projects that challenge our understanding and push us to learn new technologies. This project was conceived with that purpose in mind, aimed at deepening my familiarity with both Rust and Elasticsearch.

This application was a fun side project to learn how to work with both Rust and Elasticsearch. It fetches current weather data from the OpenWeatherMap API and pushes it into an Elasticsearch cluster. It was a great way to learn and experiment with these technologies while doing something somewhat practical. Plus it's always cool to keep track of weather trends from day to day, or month to month and build customized dashboards using Kibana. 

## Setup

### Pre-requisits:

1. An Elasticsearch instance with Kibana. I utilize Docker and [Elastic's instructions](https://www.elastic.co/guide/en/elasticsearch/reference/current/docker.html#docker-cli-run-dev-mode) with some modifications to support a real SSL certificate. 
3. An [OpenWeatherMap](https://openweathermap.org/) API key and your Zip code
4. You have [Rust installed](https://www.rust-lang.org/tools/install)
4. An enjoyment of trying silly projects like this one

### Setting up Elasticsearch

You can accomplish this in one of two ways, the first takes advantage of Elastic's Index Templates section, which basically provides a simple GUI to run through the options. The latter utilizes the Dev Tools to setup. I'll cover both, but I'd recommend the Dev tools because honestly that's quite a bit easier.

#### The GUI way


##### 1. Setting up the Index:

- On the left select the hamburger menu
- Scroll down to "Stack Management"
- Under "Data" select "Index Management" 
- From there select the "Index Templates" tab
- Select the "Create template" button
- Name your index, and create your index pattern
    - `weather_data` and `weather_data` is what I selected, but feel free to adjust as you see fit. 
- Select "Next" until you reach the "Mappings" section.
- Select the "Load JSON" option at the top right
- Open the [Weather Data Mapping](./Other-Useful-Stuff/weather_data_mapping.json) file, and copy paste that into this window. 
- Select "Load and overwrite"
- Select "Next" and "Create template"

##### 2. Setting up the Ingest Pipeline:

- On the left select the hamburger menu
- Scroll down to "Stack Management"
- Under "Ingets" select "Ingest Pipelines"
- Select "Create Pipeline" button
- Select "New Pipeline"
- Name your new Pipeline
    - I choose `weather_data_ts_fix` but you can select whatever is best for your environment. 
- Add a description if you feel the need
- Select "Import Processor" under "Processors"
- Copy and paste the [ingest pipeline provided here](./Other-Useful-Stuff/ingest_pipeline.json)
- Select "Create pipeline"

##### [Next, update and compile your version of the code!](#updating-and-compiling-your-binary)

#### The Dev Tools way

##### 1. Setting up the Index:

All of the following commands will be executed via the Dev Tools console within Kibana.

- Create your index:

```
PUT weather_data
```

- Create your mapping [using the provided mapping file](./Other-Useful-Stuff/weather_data_mapping.json)

```
PUT weather_data/_mapping
<COPY PASTA THE JSON FROM THE WEATHER_DATA_MAPPING.JSON FILE HERE>
```

- Create your ingest pipeline

```
PUT _ingest/pipeline/weather_data_ts_fix
{
  "description": "Adds timestamp to information upon inprocessing",
  "processors": [
    {
      "rename": {
        "field": "dt",
        "target_field": "@timestamp"
      }
    }
  ]
}
```

##### [Next, update and compile your version of the code!](#updating-and-compiling-your-binary)

### Updating and Compiling your Binary

- First you'll want to open up under `src` the `main.rs` file
- Then under lines 12 and 13, update the two static variables with your API key, and zip code

```rust
static OPEN_WEATHER_API: &str = "858a33787299917fd75175b58cu72e4e";
static ZIP_CODE: &str = "60606";
```

- Next under lines 17 and 18, adjust this to be your ELasticsearch URL and API key. 
    - It's important to note you'll need to add the following:
        - URL to Elasticsearch
        - Index Name
        - Pipeline Name

```rust
static ELASTIC_URL = "https://ELASTICSEARCH.URL.GOES.HERE/<INDEX NAME>/_doc?pipeline=<PIPELINE NAME>";
static ELASTIC_AUTH = "aG13OGV5N2MzenRxc3hkcnBnbmo6OHdlNzZiNTloenVxdmpyeDRzZ3Rwbgo=";
```

- Then compile your binary by running `cargo build --release` 
- You can find your newly generated binary under `./target/release/weather_to_elastic`

### Creating a Systemd Service to run the binary for funsies

- Copy the generated binary to the following location: `/usr/local/bin/`

```bash
sudo cp ./weather_to_elastic /usr/local/bin/weather_to_elastic
```

- Next copy the [weather_to_elastic.service](./Other-Useful-Stuff/weather_to_elastic.service) file to the following location: `/etc/systemd/system/`
```bash
sudo cp ./weather_to_elastic.service /etc/systemd/system/
```

- Then go ahead and enable and start the service:

```bash
sudo systemctl enable weather_to_elastic.service
sudo systemctl start weather_to_elastic.service
```

- This enables your system to always run this program when the OS starts up, saving you from having to create a cron job, though that would work fine as well. 

