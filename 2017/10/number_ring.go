package main

import (
	"fmt"
	"log"
)

type Ring struct {
	next, prev *Ring
	Value      int
}

func (r *Ring) init() *Ring {
	r.next = r
	r.prev = r
	return r
}

// Next returns the next ring element. r must not be empty.
func (r *Ring) Next() *Ring {
	return r.next
}

// Prev returns the previous ring element. r must not be empty.
func (r *Ring) Prev() *Ring {
	return r.prev
}

// Move moves n % r.Len() elements backward (n < 0) or forward (n >= 0)
// in the ring and returns that ring element. r must not be empty.
//
func (r *Ring) Move(n int) *Ring {
	switch {
	case n < 0:
		for ; n < 0; n++ {
			r = r.prev
		}
	case n > 0:
		for ; n > 0; n-- {
			r = r.next
		}
	}
	return r
}

// New creates a ring of n elements.
func New(n int) *Ring {
	if n <= 0 {
		return nil
	}
	r := new(Ring)
	//	r.Value = 0
	p := r
	for i := 1; i < n; i++ {
		p.next = &Ring{prev: p}
		p.Value = i - 1
		p = p.next
	}
	p.Value = n - 1
	p.next = r
	r.prev = p
	return r
}

func (r *Ring) String() string {
	return fmt.Sprintf("ring(%v)", r.ToSlice())
}

// Len computes the number of elements in ring r.
// It executes in time proportional to the number of elements.
//
func (r *Ring) Len() int {
	n := 0
	if r != nil {
		n = 1
		for p := r.Next(); p != r; p = p.next {
			n++
		}
	}
	return n
}

// Do calls function f on each element of the ring, in forward order.
// The behavior of Do is undefined if f changes *r.
func (r *Ring) Do(f func(int)) {
	if r != nil {
		f(r.Value)
		for p := r.Next(); p != r; p = p.next {
			f(p.Value)
		}
	}
}

// Reverse a length of the ring specified by `length` from a specific index
// Reverse should wrap around and no length > the ring length should be allowed
func (r *Ring) Reverse(length int, startingPosition int) *Ring {
	if r.Len() < length {
		log.Fatal("ring %v is shorter than %d", r, length)
	}
	slice := []int{}

	// Reverse slice by prepending values during iteration
	//	fmt.Print("capture ")
	for i := 0; i < length; i++ {
		digit := r.Move(startingPosition + i).Value
		//		fmt.Print(r.Move(startingPosition+i).Value, " ")
		slice = append([]int{digit}, slice...)
	}
	//	fmt.Printf(" -- insert at %d\n", startingPosition)

	for i, v := range slice {
		r.Move((startingPosition + i) % r.Len()).Value = v
	}

	return r
}

func (r *Ring) ToSlice() []int {
	slice := []int{}
	for i := 0; i < r.Len(); i++ {
		slice = append(slice, r.Move(i).Value)
	}
	return slice
}

func (r *Ring) EqualsSlice(slice []int) bool {
	errout := false
	for i, v := range slice {
		currentElement := r.Move(i)
		if v != currentElement.Value {
			errout = true
		}
	}
	return !errout
}
