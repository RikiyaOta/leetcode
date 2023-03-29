/*
 * @lc app=leetcode id=973 lang=javascript
 *
 * [973] K Closest Points to Origin
 */

import { MaxPriorityQueue } from "@datastructures-js/priority-queue";

// @lc code=start
/**
 * @param {number[][]} points
 * @param {number} k
 * @return {number[][]}
 */

function kClosest(points, k) {
  const queue = new MaxPriorityQueue({ priority: (point) => point.distance });

  for (const coordinate of points) {
    const [x, y] = coordinate;
    const distance = Math.pow(x, 2) + Math.pow(y, 2);

    if (queue.size() < k) {
      queue.enqueue({ coordinate, distance });
      continue;
    }

    if (queue.front().element.distance > distance) {
      queue.dequeue();
      queue.enqueue({ coordinate, distance });
    }
  }

  return queue.toArray().map(({ element }) => element.coordinate);
}
// @lc code=end
