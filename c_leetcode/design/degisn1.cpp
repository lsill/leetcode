//
// Created by lsill on 2022/8/15.
//

#include "degisn1.h"

MyCircularDeque::MyCircularDeque(int k) {
    elements.resize(k);
    capacity = k +1;
    front = tail = 0;
}

bool MyCircularDeque::insertFront(int value) {
    if (isFull()) {
        return false;
    }
    front = (front - 1 + capacity) % capacity;
    elements[front] = value;
    return true;
}

bool MyCircularDeque::insertLast(int value) {
    if (isFull()) {
        return false;
    }
    elements[tail] = value;
    tail = (tail + 1 ) %capacity;
    return true;
}

bool MyCircularDeque::deleteFront() {
    if (isEmpty()) {
        return false;
    }
    front = (front + 1) % capacity;
    return true;
}

bool MyCircularDeque::deleteLast() {
    if (isEmpty()) {
        return false;
    }
    tail = (tail - 1 + capacity) % capacity;
    return true;
}

int MyCircularDeque::getFront() {
    if (isEmpty()) {
        return -1;
    }
    return elements[front];
}

int MyCircularDeque::getRear() {
    if (isEmpty()) {
        return -1;
    }
    return elements[(tail - 1 + capacity) % capacity];
}

bool MyCircularDeque::isEmpty() {
    if (front == tail) {
        return true;
    }
    return false;
}

bool MyCircularDeque::isFull() {
    if (front == (tail + 1) % capacity) {
        return true;
    }
    return false;
}