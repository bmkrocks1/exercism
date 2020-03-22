//
// This is only a SKELETON file for the 'Linked List' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

function Node(value) {
  return {
    value,
    prev: null,
    next: null
  };
}

export class LinkedList {
  push(value) {
    if (!this.head) {
      this.head = Node(value);
      this.tail = this.head;
    }
    else {
      this.tail.next = Node(value);
      this.tail.next.prev = this.tail;
      this.tail = this.tail.next;
    }
  }

  pop() {
    if (this.tail) {
      const { value, prev } = this.tail;
      if (!prev) {
        this.tail = null;
        this.head = this.tail;
      }
      else {
        prev.next = null;
        this.tail = prev;
      }
      return value;
    }
    else {
      throw 'Cannot pop on an empty LinkedList.';
    }
  }

  shift() {
    if (this.head) {
      const { value, next } = this.head;
      if (!next) {
        this.tail = null;
        this.head = this.tail;
      }
      else {
        next.prev = null;
        this.head = next;
      }
      return value;
    }
    else {
      throw 'Cannot shift on an empty LinkedList.';
    }
  }

  unshift(value) {
    if (!this.head) {
      this.head = Node(value);
      this.tail = this.head;
    }
    else {
      this.head.prev = Node(value);
      this.head.prev.next = this.head;
      this.head = this.head.prev;
    }
  }

  delete(value) {
    let current = this.head;
    while (current) {
      if (current.value === value) {
        const { value, prev, next } = current;
        if (!prev) {
          // current is head
          this.shift();
        }
        else if (!next) {
          // current is tail
          this.pop();
        } else {
          // current is somewhere in the middle
          prev.next = next;
          next.prev = prev;
        }
        break;
      }
      current = current.next;
    }
  }

  count() {
    let ctr = 0;
    let current = this.head;
    while (current) {
      ctr++;
      current = current.next;
    }
    return ctr;
  }
}
