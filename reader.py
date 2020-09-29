

from kafka import KafkaConsumer
consumer = KafkaConsumer('foobar',bootstrap_servers='192.168.1.80:9092')

print("connected ? ")
for msg in consumer:
    print (msg)

