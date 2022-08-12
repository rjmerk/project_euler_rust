/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in
words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example,
342 (three hundred and forty-two) contains 23 letters and
115 (one hundred and fifteen) contains 20 letters.

The use of "and" when writing out numbers is in compliance with British usage.
*/

pub fn solve()
{

}

fn number_written_out_in_words(n: i32) -> String
{
    assert!(n < 1000);
    if n > 99 {
        String::from("")
    } else {
        two_digit_number_written_as_word(n)
    }
}


fn two_digit_number_written_as_word(n: i32) -> String
{
    println!("{n}");
    String::from("")
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn check_if_example_works()
    {
        assert_eq!(number_written_out_in_words(42), "forty-two");
        assert_eq!(number_written_out_in_words(342), "three hundred and forty-two");
        assert_eq!(number_written_out_in_words(115), "one hundred and fifteen");
    }
}
