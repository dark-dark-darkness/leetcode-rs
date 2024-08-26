# 690. 员工的重要性
# https://leetcode.cn/problems/employee-importance

"""
# Definition for Employee.
class Employee:
    def __init__(self, id: int, importance: int, subordinates: List[int]):
        self.id = id
        self.importance = importance
        self.subordinates = subordinates
"""

class Employee:
    def __init__(self, id: int, importance: int, subordinates: List[int]):
        self.id = id
        self.importance = importance
        self.subordinates = subordinates

class Solution:
    def getImportance(self, employees: List['Employee'], id: int) -> int:
        es = [None for i in range(2000 + 1)]
        for e in employees:
            es[e.id] = e 
        return self.getImportanceCore(es,id)
        
    def getImportanceCore(self,employees:['Employee'],id:int) -> int:
        acc = employees[id].importance

        for e in employees[id].subordinates:
            acc += self.getImportanceCore(employees,employees[e].id)

        return acc