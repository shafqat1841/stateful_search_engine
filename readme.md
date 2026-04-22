# For practicing lifetimes, references, and concurrency.

## Project: The "Super-Fast" Shared Cache
    Instead of just reading a file once, you will build a system that stores "hot" data in memory. If two different parts of your program (or different threads) need the same data, they should point to the same original buffer without copying it.

---------------------------------------------------------------------------------------------------------------------

## Why this is the "Next Level":
1, Multiple Lifetimes: You will have to manage the lifetime of the data and the lifetime of the cache entries simultaneously.

2, Interior Mutability: You will need to use Arc and Mutex or RwLock to share references across threads safely.

3, The "Double-Reference" Challenge: You will build a struct that holds a reference to a key and a reference to a value, both stored in a central "Arena" or buffer.

---------------------------------------------------------------------------------------------------------------------

## Key Rust Features You Will Practice
1, Arc<T> (Atomic Reference Counting): For sharing ownership of the data buffer across threads.

2, RwLock<T> (Read-Write Lock): Allowing multiple threads to read the cache at once, but only one to write to it.

3, Advanced Lifetimes: Defining how a CacheEntry<'a> is tied to the SharedBuffer.

4, Traits and Generics: Making your cache work for any data type, not just log lines.

---------------------------------------------------------------------------------------------------------------------

## Step-by-Step Functionality

### Step 1: The Arena (The Source of Truth)
    Create a SharedBuffer struct that holds a large block of memory (like your Mmap). Wrap it in an Arc so multiple threads can "own" the view of this memory.

### Step 2: The Cache Structure
    Build a Cache<'a> struct. It should store a HashMap<&'a str, CacheEntry<'a>>.
        1, The Key is a reference to a string in the buffer.
        2, The Value is a reference to the data in the buffer.

### Step 3: Implement LRU Logic
    Every time someone accesses a key, you move it to the "front" of the list. If the cache exceeds a certain size (e.g., 100 items), you Evict the oldest item.

### Step 4: The Concurrency Challenge
    Make the cache accessible by two different threads. One thread "searches" for IP addresses (like your current project), and the other "counts" occurrences. Both must borrow from the same Arc<Mmap>.