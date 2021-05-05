"""
# Definition for Employee.
class Employee:
    def __init__(self, id: int, importance: int, subordinates: List[int]):
        self.id = id
        self.importance = importance
        self.subordinates = subordinates
"""

from collections import deque

class Solution:
	def getImportance(self, employees, id: int) -> int:
		map = {}
		for employee in employees:
			map[employee.id] = employee

		que = deque([id])
		ans = 0
		while len(que) != 0:
			e = map.get(que.popleft())
			if not e:
				continue
			ans += e.importance
			for c in e.subordinates:
				que.append(c)
		return ans
