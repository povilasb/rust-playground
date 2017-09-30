=====
About
=====

The previous challenges covered Insertion Sort, which is a simple and intuitive
sorting algorithm with an average case performance of . In these next few
challenges, we're covering a divide-and-conquer algorithm called Quicksort
(also known as Partition Sort).

Step 1: Divide
Choose some pivot element, , and partition your unsorted array, , into three
smaller arrays: , , and , where each element in , each element in , and each
element in .

Challenge
Given  and , partition  into , , and  using the Divide instructions above. Then
print each element in  followed by each element in , followed by each element
in  on a single line. Your output should be space-separated.

Note: There is no need to sort the elements in-place; you can create two lists
and stitch them together at the end.

Input Format

The first line contains  (the size of ).  The second line contains
space-separated integers describing  (the unsorted array). The first integer
(corresponding to ) is your pivot element, .

Constraints

All elements will be unique.
Multiple answer can exists for the given test case. Print any one of them.

Output Format

On a single line, print the partitioned numbers (i.e.: the elements in , then
the elements in , and then the elements in ). Each integer should be separated
by a single space.

Sample Input

5
4 5 3 7 2

Sample Output

3 2 4 5 7
