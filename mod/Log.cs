using BepInEx.Logging;
using System.Runtime.CompilerServices;
using System.IO;

namespace WeatherIndex
{
    internal static class Log
    {
        private static ManualLogSource _logSource;

        internal static void Init(ManualLogSource logSource)
        {
            _logSource = logSource;
        }

        private static string Format(object data, string file, int line)
        {
            string fileName = Path.GetFileName(file);
            return $"[{fileName}:{line}] {data}";
        }

        internal static void Debug(
            object data,
            [CallerFilePath] string file = "",
            [CallerLineNumber] int line = 0)
            => _logSource.LogDebug(Format(data, file, line));

        internal static void Error(
            object data,
            [CallerFilePath] string file = "",
            [CallerLineNumber] int line = 0)
            => _logSource.LogError(Format(data, file, line));

        internal static void Fatal(
            object data,
            [CallerFilePath] string file = "",
            [CallerLineNumber] int line = 0)
            => _logSource.LogFatal(Format(data, file, line));

        internal static void Info(
            object data,
            [CallerFilePath] string file = "",
            [CallerLineNumber] int line = 0)
            => _logSource.LogInfo(Format(data, file, line));

        internal static void Message(
            object data,
            [CallerFilePath] string file = "",
            [CallerLineNumber] int line = 0)
            => _logSource.LogMessage(Format(data, file, line));

        internal static void Warning(
            object data,
            [CallerFilePath] string file = "",
            [CallerLineNumber] int line = 0)
            => _logSource.LogWarning(Format(data, file, line));
    }
}
