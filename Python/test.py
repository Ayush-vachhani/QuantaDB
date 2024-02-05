import sqlite3
from time import time


def insert_rows(db_path, num_rows=100000):
    connection = sqlite3.connect(db_path)
    cursor = connection.cursor()

    # Using a transaction to speed up insert operations
    connection.execute('BEGIN')
    for i in range(num_rows):
        cursor.execute('INSERT INTO test (id, name) VALUES (?, ?)', (i, f'Name {i}'))
    connection.commit()
    connection.close()


def read_rows(db_path):
    connection = sqlite3.connect(db_path)
    cursor = connection.cursor()
    cursor.execute('SELECT * FROM test')
    rows = cursor.fetchall()
    connection.close()
    return rows


def delete_all_records(db_path):
    connection = sqlite3.connect(db_path)
    cursor = connection.cursor()
    cursor.execute('DELETE FROM test')
    connection.commit()
    connection.close()


def measure_time(function, *args):
    start_time = time()
    function(*args)
    end_time = time()
    print(f"Time taken to execute {function.__name__}: {(end_time - start_time) * 1000:.2f} ms")


path = 'test.sqlite3'

# Measure time taken to insert rows
measure_time(insert_rows, path)

# Measure time taken to read rows
measure_time(read_rows, path)

measure_time(delete_all_records, path)
