import sqlite3
import json
from time import time

class Producer:
    def __init__(self, id, name, price, adjacency_list):
        self.id = id
        self.name = name
        self.price = price
        self.adjacency_list = adjacency_list
Types = str

def insert_rows(db_path, num_rows=100_000):
    connection = sqlite3.connect(db_path)
    cursor = connection.cursor()

    # Using a transaction to speed up insert operations
    connection.execute('BEGIN')
    cursor.execute('DROP TABLE IF EXISTS test')
    cursor.execute('CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT, price REAL, adjacency_list TEXT)')

    for i in range(num_rows):
        # Placeholder - assuming you have logic to generate adjacency lists
        adjacency_list = {}
        serialized_adj_list = json.dumps(adjacency_list)

        cursor.execute('INSERT INTO test (id, name, price, adjacency_list) VALUES (?, ?, ?, ?)',
                       (i, f'Name {i}', 10.0, serialized_adj_list))

    connection.commit()
    connection.close()


def read_rows(db_path):
    connection = sqlite3.connect(db_path)
    cursor = connection.cursor()
    cursor.execute('SELECT * FROM test')
    rows = cursor.fetchall()

    producers = []
    for row in rows:
        adjacency_list = json.loads(row[3])
        producer = Producer(id=row[0], name=row[1], price=row[2], adjacency_list=adjacency_list)
        producers.append(producer)

    connection.close()
    return producers

def delete_all_records(db_path):
    connection = sqlite3.connect(db_path)
    cursor = connection.cursor()
    cursor.execute('DROP TABLE IF EXISTS test')
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
