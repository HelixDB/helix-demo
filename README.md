# Setup

## HelixCLI installation

```bash
curl -sSL https://install.helix-db.com | bash
```

## Install Helix

```bash
helix install
```

# Run Queries

Take a look at the queries in `./helixdb-cfg` to see what is being used.

> cd into `helix-demo` if you haven't already
```bash
helix deploy --local
```

# Setup python environment

```bash
python3 -m venv venv
source venv/bin/activate
pip install helix-py
```

# Send Requests

```bash
python3 ./create_data.py
```

> Run this one first to create the data

```bash
python3 ./get_patient.py
```

> Run this one second to get a patient

```bash
python3 ./get_visit_by_date.py
```

> Run this one third to get visits by date

```bash
python3 ./get_patients_visits_in_previous_month.py
```

> Run this one fourth to get patients visits in previous month