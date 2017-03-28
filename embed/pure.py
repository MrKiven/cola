# -*- coding: utf-8 -*-

import threading


def process():
    for _ in xrange(5000000):
        pass


def run():
    threads = []
    for _ in xrange(10):
        threads.append(threading.Thread(target=process))

    [thread.start() for thread in threads]
    [thread.join() for thread in threads]

    print "Done"


if __name__ == "__main__":
    run()
