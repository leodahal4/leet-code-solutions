class Solution:
    def isPalindrome(self, x: int) -> bool:
        if "-" in str(x):
            return False
        
        rev = int(str(x)[::-1])
        return rev == x

check = Solution()
if check.isPalindrome(121):
    print("That's a palindrome")
else:
    print("That isn't a palindrome")

##This is the comment added by npoudelp
