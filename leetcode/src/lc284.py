class PeekingIterator:
		def __init__(self, iterator):
				"""
				Initialize your data structure here.
				:type iterator: Iterator
				"""
				self.iter = iterator
				self.peekVal = None

		def peek(self):
				"""
				Returns the next element in the iteration without advancing the iterator.
				:rtype: int
				"""
				if self.peekVal is not None:
					return self.peekVal
				val = self.iter.next()
				self.peekVal = val
				return val

		def next(self):
				"""
				:rtype: int
				"""
				if self.peekVal is not None:
					val = self.peekVal
					self.peekVal = None
					return val
				return self.iter.next()

		def hasNext(self):
				"""
				:rtype: bool
				"""
				return self.iter.hasNext() or self.peekVal is not None
