[![Build Status](https://travis-ci.org/jbelmont/csv-to-json.svg?branch=master)](https://travis-ci.org/jbelmont/csv-to-json)

# An exploration on converting a csv file using minimal library support in Rust

This repository create a json output using a CSV file from [data.gov](https://catalog.data.gov/)

Here is the following csv data for reference:

*demographic_statistics_by_zip_code.csv*:

```csv
JURISDICTION NAME,COUNT PARTICIPANTS,COUNT FEMALE,PERCENT FEMALE,COUNT MALE,PERCENT MALE,COUNT GENDER UNKNOWN,PERCENT GENDER UNKNOWN
10001,44,22,0.5,22,0.5,0,0
10002,35,19,0.54,16,0.46,0,0
...
```

## Running the Rust Project

In order to run the the csv-to-json project run the following command:

```bash
cargo run demographic_statistics_by_zip_code.csv

"[{\"jurisdiction_name\":10001,\"count_participants\":44,\"count_female\":22,\"percent_female\":0.5,\"count_male\":22,\"percent_male\":0.5,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10002,\"count_participants\":35,\"count_female\":19,\"percent_female\":0.54,\"count_male\":16,\"percent_male\":0.46,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10003,\"count_participants\":1,\"count_female\":1,\"percent_female\":1.0,\"count_male\":0,\"percent_male\":0.0,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10004,\"count_participants\":0,\"count_female\":0,\"percent_female\":0.0,\"count_male\":0,\"percent_male\":0.0,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10005,\"count_participants\":2,\"count_female\":2,\"percent_female\":1.0,\"count_male\":0,\"percent_male\":0.0,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10006,\"count_participants\":6,\"count_female\":2,\"percent_female\":0.33,\"count_male\":4,\"percent_male\":0.67,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10007,\"count_participants\":1,\"count_female\":0,\"percent_female\":0.0,\"count_male\":1,\"percent_male\":1.0,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},{\"jurisdiction_name\":10009,\"count_participants\":2,\"count_female\":0,\"percent_female\":0.0,\"count_male\":2,\"percent_male\":1.0,\"count_gender_unknown\":0,\"percent_gender_unknown\":0.0},
...
```
