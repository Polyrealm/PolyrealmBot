using System.Collections.Concurrent;
using LiteDB;

namespace PolyrealmBot.Handlers.Entities
{
    public class GuildEntity : BaseEntity
    {
        public string Prefix { get; set; }
    }
}
