using Discord.WebSocket;
using Qmmands;

namespace PolyrealmBot.Modules.Custom
{
    public sealed class Context : ICommandContext{
        public SocketGuild Guild {get;}
        public SocketGuildUser User {get;}
        public DiscordSocketClient Client {get;}
        public SocketTextChannel Channel {get;}
        public SocketUserMessage Message {get;}

        public Context(DiscordSocketClient client, SocketUserMessage message){
            Client = client;
            Message = message;
            Channel = message.Channel as SocketTextChannel;
            User = message.Author as SocketGuildUser;
            Guild = (message.Channel as SocketGuildChannel).Guild;
        }
    }
}
