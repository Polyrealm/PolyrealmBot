using System;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using PolyrealmBot.Handlers.Entities;

namespace PolyrealmBot.Handlers
{
    public class DiscordHandler
    {
        private string _prefix;
        private SettingsEntity _settings;
        private IServiceProvider _serviceProvider;

        private DiscordSocketClient _socketClient;
        private CommandService _commandService;

        public DiscordHandler(DbHandler dbHandler, DiscordSocketClient socketClient, CommandService commandService)
        {
            _socketClient = socketClient;
            _commandService = commandService;

            var settings = dbHandler.Get<SettingsEntity>("Settings");

            if (!(settings is null))
            {
                _prefix = settings.Prefix;
                _settings = settings;
                return;
            }

            // TODO: Logger
            Console.Write("Enter Token: ");
            var token = Console.ReadLine();
            Console.Write("Enter Prefix: ");
            var prefix = Console.ReadLine();

            settings = new SettingsEntity{
                Id = "Settings",
                Token = token,
                Prefix = prefix
            };

            _prefix = prefix;
            dbHandler.Save(settings);
            Console.Write("Settings created!");
            _settings = settings;
        }

        public async Task InitializeAsync(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;

            // TODO: Event Linking

            await _socketClient.LoginAsync(TokenType.Bot, _settings.Token);
            await _socketClient.StartAsync();
        }
    }
}
