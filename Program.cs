using System;
using System.Threading.Tasks;
using Discord.Commands;
using Discord.WebSocket;
using Microsoft.Extensions.DependencyInjection;
using PolyrealmBot.Handlers;

namespace PolyrealmBot
{
    class Program
    {
        private static async Task Main(string[] args)
        {
            var services = new ServiceCollection()
                .AddSingleton<CommandService>()
                .AddSingleton<DiscordSocketClient>()
                .AddSingleton<DbHandler>()
                .AddSingleton<DiscordHandler>();

            var provider = services.BuildServiceProvider();
            await provider.GetRequiredService<DiscordHandler>().InitializeAsync(provider);

            await Task.Delay(-1);
        }
    }
}
