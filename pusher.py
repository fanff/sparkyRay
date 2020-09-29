

from kafka import KafkaProducer
producer = KafkaProducer(bootstrap_servers='192.168.1.80:9092')

for _ in range(10):
    print("sending")
    producer.send('foobar', b'some_message_bytes')
    producer.flush()
