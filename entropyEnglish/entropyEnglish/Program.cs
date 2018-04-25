using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Text.RegularExpressions;
using System.IO;

namespace entropyEnglish
{
    public static class StringExtension
    {
        public static IEnumerable<string> Ngram(this string input, int n)
        {
            int index = input.Length - n + 1;

            while (index-- > 0)
                yield return input.Substring(index, n).ToLower();
        }
    }

    class Program
    {
        private static (IEnumerable<IGrouping<string, string>> rankedNgrams, long amount) Ngrams(ref string lesMiserables, int n)
        {
            //split book string into a array of words
            var rankedNgrams = lesMiserables.Split(' ')
                //map each word to ngrams
                .SelectMany(x => x.Ngram(n))
                //group by occurences of ngrams and rank them
                .GroupBy(ngram => ngram).OrderByDescending(x => x.Count());

            //return tuple of ranked ngrams and amount of total ngrams
            return (rankedNgrams.Take(10), rankedNgrams.Sum(x => x.Count()));
        }

        private static double EntropyFromNgrams(ref IEnumerable<IGrouping<string, string>> rankedNgrams, long total, int n)
        {
            //map ngrams to entropy
            return rankedNgrams.Select(ng =>
            {
                double startValue = ng.Count() / (double)total;

                if (startValue == 0)
                    return 0;
                else
                    return startValue * Math.Log(startValue, 2);
            }).Aggregate(0.0d, (acc, x) => acc + x) / n * -1;
        }

        static void Main(string[] args)
        {
            if (!File.Exists("lesMiserables.txt"))
            {
                Console.WriteLine("Fetching Les Misérables.");

                using (WebClient client = new WebClient())
                {
                    File.WriteAllText("lesMiserables.txt", client.DownloadString("http://www.gutenberg.org/files/135/135-0.txt"));
                }
            }

            var lesMiserables = File.ReadAllText("lesMiserables.txt");

            //remove all characters that are not words or spaces
            lesMiserables = Regex.Replace(lesMiserables, "[^\\w ]+", "");
            //remove all multiple spaces
            lesMiserables = Regex.Replace(lesMiserables, " +(?= )", "");

            var nGramsOfSizeOne = Ngrams(ref lesMiserables, 1);
            var nGramsOfSizeTwo = Ngrams(ref lesMiserables, 2);
            var nGramsOfSizeThree = Ngrams(ref lesMiserables, 3);

            Console.WriteLine($"Entropy for ngrams of size one: {EntropyFromNgrams(ref nGramsOfSizeOne.rankedNgrams, nGramsOfSizeOne.amount, 1)}.");
            Console.WriteLine($"Entropy for ngrams of size two: {EntropyFromNgrams(ref nGramsOfSizeTwo.rankedNgrams, nGramsOfSizeTwo.amount, 2)}.");
            Console.WriteLine($"Entropy for ngrams of size three: {EntropyFromNgrams(ref nGramsOfSizeThree.rankedNgrams, nGramsOfSizeThree.amount, 3)}.");

            Console.ReadLine();
        }
    }
}
