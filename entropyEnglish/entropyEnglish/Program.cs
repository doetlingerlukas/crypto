using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Text.RegularExpressions;
using System.IO;
using System.Threading.Tasks;
using System.Diagnostics;

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
        internal class NgramTuple<T>
        {
            public T Ngram { get; set; }
            public long NgramCount { get; set; } = 0;

            public NgramTuple(T ngram)
            {
                Ngram = ngram;
            }

            public NgramTuple(T ngram, long ngramCount)
            {
                Ngram = ngram;
                NgramCount = ngramCount;
            }
        }

        private static NgramTuple<IEnumerable<IGrouping<string, string>>> Ngrams(string lesMiserables, int n)
        {
            var ngrams = new NgramTuple<List<string>>(new List<string>());

            int size = lesMiserables.Length / Environment.ProcessorCount;

            var lmSplitted = Enumerable.Range(1, Environment.ProcessorCount / 2)
                .Select(x => lesMiserables.Substring(size * (x - 1), size * x)).ToArray();

            Parallel.For(0, lmSplitted.Length, i =>
            {
                var ngramsInternal = new NgramTuple<List<string>>(new List<string>());

                foreach (var ngram in lmSplitted[i].Ngram(n))
                {
                    ngramsInternal.Ngram.Add(ngram);
                    ngramsInternal.NgramCount++;
                }

                lock (ngrams)
                {
                    ngrams.Ngram.AddRange(ngramsInternal.Ngram);
                    ngrams.NgramCount += ngramsInternal.NgramCount;
                }
            });

            //group by occurences of ngrams and rank them
            var rankedNgrams = ngrams.Ngram.GroupBy(ngram => ngram).OrderByDescending(x => x.Count());

            //return tuple of ranked ngrams and amount of total ngrams
            return new NgramTuple<IEnumerable<IGrouping<string, string>>>(rankedNgrams, ngrams.NgramCount);
        }

        private static double EntropyFromNgrams(IEnumerable<IGrouping<string, string>> rankedNgrams, long total, int n)
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

            Console.WriteLine("Calculating entropies...");

            Stopwatch stopwatch = new Stopwatch();
            stopwatch.Start();

            Parallel.Invoke(() =>
            {
                var nGramsOfSizeOne = Ngrams(lesMiserables, 1);
                Console.WriteLine($"Entropy for n-grams of size one: {EntropyFromNgrams(nGramsOfSizeOne.Ngram, nGramsOfSizeOne.NgramCount, 1)}.");
            }, () =>
            {
                var nGramsOfSizeTwo = Ngrams(lesMiserables, 2);
                Console.WriteLine($"Entropy for n-grams of size two: {EntropyFromNgrams(nGramsOfSizeTwo.Ngram, nGramsOfSizeTwo.NgramCount, 2)}.");
            }, () =>
            {
                var nGramsOfSizeThree = Ngrams(lesMiserables, 3);
                Console.WriteLine($"Entropy for n-grams of size three: {EntropyFromNgrams(nGramsOfSizeThree.Ngram, nGramsOfSizeThree.NgramCount, 3)}.");
            });

            stopwatch.Stop();
            Console.WriteLine($"Completed in {stopwatch.ElapsedMilliseconds}ms");

            Console.ReadLine();
        }
    }
}
