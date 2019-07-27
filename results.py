import os
import pandas as pd
import psycopg2
from sqlalchemy import create_engine
import matplotlib.pyplot as plt

###################################
# EDIT THESE CONSTANTS
###################################

DB_PASSWORD = ""
AUTHOR_NAME = "Philipp Krones"

ANALYZER_NAME = f"beta/flip1995-clippy"
ANALYZER_VERSION = "0.0.1"
CORPUS_NAME = "crates-io/0.0.1"

###################################
# END EDIT SECTION
###################################

# Canonical SQL query to get job-specific results back.
JOB_QUERY = """
SELECT *
FROM   result,
       commit_corpus
WHERE  result.commit_hash = commit_corpus.commit_hash
       AND analyzer_name = %(analyzer_name)s
       AND analyzer_version = %(analyzer_version)s
       AND corpus_name = %(corpus_name)s
"""

QUERY_PARAMS = {
    "corpus_name": CORPUS_NAME,
    "analyzer_name": ANALYZER_NAME,
    "analyzer_version": ANALYZER_VERSION,
}

# Connect to PostgreSQL host and query for job-specific results
engine = create_engine(f"postgresql://notebook_user:{DB_PASSWORD}@db.r2c.dev/postgres")
job_df = pd.read_sql(JOB_QUERY, engine, params=QUERY_PARAMS)


def get_results(row):
    switcher = {
        "None": 0,
        "Warning": 1,
        "Error": 2,
        "ICE": 3,
    }
    return switcher.get(row.extra["error_type"], 0)


job_df["error_type"] = job_df.apply(get_results, axis=1)

job_df.hist(column="error_type", bins=4)
plt.show()

