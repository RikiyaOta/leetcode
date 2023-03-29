/*
 * @lc app=leetcode id=1046 lang=javascript
 *
 * [1046] Last Stone Weight
 */

import { MaxPriorityQueue } from "@datastructures-js/priority-queue";

// @lc code=start
/**
 * @param {number[]} stones
 * @return {number}
 */

function lastStoneWeight(stones) {
  if (stones.length === 0) return 0;
  if (stones.length === 1) return stones[0];

  const queue = new MaxPriorityQueue();

  for (const stone of stones) {
    queue.enqueue(stone);
  }

  while (queue.size() >= 2) {
    console.log({ queue: queue.toArray() });
    const n1 = queue.dequeue().element;
    const n2 = queue.dequeue().element;

    console.log({ n1, n2 });

    // note: n1 >= n2

    if (n1 > n2) {
      queue.enqueue(n1 - n2);
    }
  }

  if (queue.size() === 0) {
    return 0;
  } else {
    return queue.front().element;
  }
}

// @lc code=end
