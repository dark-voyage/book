## Hash Maplarda bog'langan qiymatlarga ega kalitlarni saqlash

Umumiy to'plamlarimizning oxirgisi *hash map*. `HashMap<K, V>` turi `K` turidagi kalitlarning `V` turidagi qiymatlarga xaritasini *xeshlash funksiyasi* yordamida saqlaydi, bu kalit va qiymatlarni xotiraga qanday joylashtirishini belgilaydi. Ko'pgina dasturlash tillari bunday ma'lumotlar strukturasini qo'llab-quvvatlaydi, lekin ular ko'pincha bir nechtasini nomlash uchun hash, map, object, hash table, dictionary, yoki associative array kabi boshqa nomlardan foydalanadilar.

Hash maplar ma'lumotlarni vektorlar bilan bo'lgani kabi indeks yordamida emas, balki har qanday turdagi kalit yordamida qidirmoqchi bo'lsangiz foydali bo'ladi. Misol uchun, o'yinda siz har bir jamoaning balini hesh-mapda kuzatib borishingiz mumkin, unda har bir kalit jamoaning nomi va qiymatlar har bir jamoaning ballidir. Jamoa nomi berilgan bo'lsa, siz uning ballini olishingiz mumkin.

Ushbu bo'limda biz hesh-mapllarining asosiy API-ni ko'rib chiqamiz, ammo yana ko'plab foydali funksiyalar standart kutubxona tomonidan `HashMap<K, V>` da belgilangan funksiyalarda yashiringan.
Har doimgidek, qo'shimcha ma'lumot olish uchun standart kutubxona texnik hujjatlarini tekshiring.

### Yangi Hash Map yaratish

Bo'sh hesh mapni yaratishning bir usuli - `new` dan foydalanish va `insert` bilan elementlarni qo'shish. 8-20 ro'yxatda biz nomlari *Yashil* va *Sariq* bo'lgan ikkita jamoaning ballarini kuzatib boramiz. Yashil jamoa 10 ball bilan, Sariq jamoa esa 50 ball bilan boshlanadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-20: Yangi hesh mapni yaratish va ba'zi kalitlar va qiymatlarni kiritish</span>

E'tibor bering, biz birinchi navbatda standart kutubxonaning to'plamlar qismidagi `HashMap` dan foydalanishimiz kerak. Bu quyidagicha bo'ladi `use std::collections::HashMap;`. Bizning uchta keng tarqalgan to'plamlarimiz orasida bu eng kam qo'llaniladi, shuning uchun u muqaddimada avtomatik ravishda kiritilgan funtsiyalarga kiritilmagan. Hash Maplar standart kutubxonadan ham kamroq qo'llab-quvvatlanadi; masalan, ularni yaratish uchun o'rnatilgan makros mavjud emas.

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type as each other, and all of
the values must have the same type.

### Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-21.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Listing 8-21: Accessing the score for the Blue team
stored in the hash map</span>

Here, `score` will have the value that’s associated with the Blue team, and the
result will be `10`. The `get` method returns an `Option<&V>`; if there’s no
value for that key in the hash map, `get` will return `None`. This program
handles the `Option` by calling `copied` to get an `Option<i32>` rather than an
`Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` doesn't
have an entry for the key.

We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

This code will print each pair in an arbitrary order:

```text
Yellow: 50
Blue: 10
```

### Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>

We aren’t able to use the variables `field_name` and `field_value` after
they’ve been moved into the hash map with the call to `insert`.

If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We’ll talk more about these issues in
the [“Validating References with
Lifetimes”][validating-references-with-lifetimes]<!-- ignore --> section in
Chapter 10.

### Updating a Hash Map

Although the number of key and value pairs is growable, each unique key can
only have one value associated with it at a time (but not vice versa: for
example, both the Blue team and the Yellow team could have value 10 stored in
the `scores` hash map).

When you want to change the data in a hash map, you have to decide how to
handle the case when a key already has a value assigned. You could replace the
old value with the new value, completely disregarding the old value. You could
keep the old value and ignore the new value, only adding the new value if the
key *doesn’t* already have a value. Or you could combine the old value and the
new value. Let’s look at how to do each of these!

#### Overwriting a Value

If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-23 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Listing 8-23: Replacing a value stored with a particular
key</span>

This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Adding a Key and Value Only If a Key Isn’t Present

It’s common to check whether a particular key already exists in the hash map
with a value then take the following actions: if the key does exist in the hash
map, the existing value should remain the way it is. If the key doesn’t exist,
insert it and a value for it.

Hash maps have a special API for this called `entry` that takes the key you
want to check as a parameter. The return value of the `entry` method is an enum
called `Entry` that represents a value that might or might not exist. Let’s say
we want to check whether the key for the Yellow team has a value associated
with it. If it doesn’t, we want to insert the value 50, and the same for the
Blue team. Using the `entry` API, the code looks like Listing 8-24.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Listing 8-24: Using the `entry` method to only insert if
the key does not already have a value</span>

The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-24 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.

#### Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-25 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Listing 8-25: Counting occurrences of words using a hash
map that stores words and counts</span>

This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. You might see
the same key/value pairs printed in a different order: recall from the
[“Accessing Values in a Hash Map”][access]<!-- ignore --> section that
iterating over a hash map happens in an arbitrary order.

The `split_whitespace` method returns an iterator over sub-slices, separated by
whitespace, of the value in `text`. The `or_insert` method returns a mutable
reference (`&mut V`) to the value for the specified key. Here we store that
mutable reference in the `count` variable, so in order to assign to that value,
we must first dereference `count` using the asterisk (`*`). The mutable
reference goes out of scope at the end of the `for` loop, so all of these
changes are safe and allowed by the borrowing rules.

### Hashing Functions

By default, `HashMap` uses a hashing function called *SipHash* that can provide
resistance to Denial of Service (DoS) attacks involving hash
tables[^siphash]<!-- ignore -->. This is not the fastest hashing algorithm
available, but the trade-off for better security that comes with the drop in
performance is worth it. If you profile your code and find that the default
hash function is too slow for your purposes, you can switch to another function
by specifying a different hasher. A *hasher* is a type that implements the
`BuildHasher` trait. We’ll talk about traits and how to implement them in
Chapter 10. You don’t necessarily have to implement your own hasher from
scratch; [crates.io](https://crates.io/)<!-- ignore --> has libraries shared by
other Rust users that provide hashers implementing many common hashing
algorithms.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Summary

Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:

* Given a list of integers, use a vector and return the median (when sorted,
  the value in the middle position) and mode (the value that occurs most often;
  a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to
  the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
  that start with a vowel have “hay” added to the end instead (“apple” becomes
  “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in a company. For example, “Add Sally to
  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.

The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
