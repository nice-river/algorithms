#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "cabcac";
        let ans = "cac";
        assert_eq!(Solution::last_substring(s.to_string()), ans);
    }

    #[test]
    fn test1() {
        let s = "caacaacab";
        let ans = "cab";
        assert_eq!(Solution::last_substring(s.to_string()), ans);
    }

    #[test]
    fn test2() {
        let s = "xxbbxxbx";
        let ans = "xxbx";
        assert_eq!(Solution::last_substring(s.to_string()), ans);
    }

    #[test]
    fn test3() {
        let s = "pgtrdjdpscrwjifnrcttyruighgygsuvlhxpckkeahrupvnhlnpulyogsbktcuxnmnbmgadksxdjunqvmzyujynwzevtstjvzkddxjjmbgxfueteeuktvcbvypbdnzostbwxmxdwomguuymexfrrwuvuglgwmmwpkrqrpuzvjujksdwopsqlsrfgyzhymfgejuwhyvoqoxluvsdnmkglypoozrcgnzchpurezauixujddjjawqiaasvhrhfbhsqutgskudpcbqkkrcagrtalnsecxmlbiysgabvjbwpfufiwqdsnwbutashtejpmcypfztbgzuxwfcpkwdzhvfxbtvdvdaufjpqgfgxufhsopvwmgekcjejdlgxtdghmyzvopqkdzpuudyunnafvaeyshluuujkqncyigweelxmvgiaegonqmouaxwaqxhnnvaeuppsritdsymdwonbooswiolhbacpnehyvbhekxqxuvpnaxrfgeyixmurlgszqrotqynvtlbpjhhwdkbneiutmwutiyqegkxsrgldqvziysvihgbplhiumonlzlfzbuavbygmltdnfwjbspfpmzkfryjjpswihwixmbeodfpnmytccdvjnctzkxuqrvgjehodfdhpconhfpnatzzxyqkzjttgnwvxcfwyhlajvuhjonbpkvbbahlbybvvnhrwnnpnagtnnqcewzdzsxjgfoqbipohzsgpmyhqgvvnffjumirbhpbfletvcglthgwdvkgczucreovnfpbugyzuugqodpgsylfwyjucfnxjbghurnrkkuwsjfxeoxyzvltwhzyuechbfwstfovqgxngcwsmqsbzasrfrdvjcgedacvviihnzlijbaotpkvzmfejtfsnljthmzfhsrtieqlfhuatdwhcvsoxyphqsoxaqiqjcbuldswtjsukvcoowyfgmixswlyvkllvdtnerfisymrwgtfleupfxxswdlhvioeilstdlqfckoxpohcgnfannhbeeyzthhmszdphsbhjhxbwmjcknyrcsxgxcgnrvqnrlpqqwilhradmbowdvpusmombllpjikbuokakqzovmtmanaavururxjjxvrdnwabfwchkscmmmyixmjtogqgowbqlymnjcalxkmolmhyalvaftbcsidbmyfxshzdbwsfkvlnunvxzadljqngnnhjmlhfpmwnhvvwaokxxyfslhgggpjnhkyzdqgjdsrtaohqaryixzpyrkjumnphvfpsldrcjhxqjisogvybchagdnoezpvehagcqxarruqncmjifvxxvpbsygkzttmnykeptvbxmjsewbppcciomuyuowzbankzklnnxmnxiztfpfcnddaghyuxgaxmgphspocmpcucxkazeakifbsvnatqcvbeywbxsocgyxiawvmljwmokhbpzvijuckbqrjsdtntlriqgyjznvkkeoszsqsonjlsmxhawqqbdwixvnyemiypirwnoyhvfhbbbkcuyuuepobvrspotbbuvtmpykalufsvizvcelblhokldaymsgqasbobwyxnmcqvzqyldgmimsolloudxkjmiyekaiierxwwflmanqkovzxewrrfipzrclqvedftsuswanhfbcmjigxuxaguasfjypjjrgddnwnrfoazbvezxopumretyqfzqyosmvqjpqnuodylrkucrbhtrljryryewvfbnfwjldgjidvucotlvnsvqyjqwfmqhydttjaapqfdmvcytqpcjkqqrhwfiimbvlkvnvtghrggkjppaduhqpdneeejpqrxbnnenvobcgreldbnzpzrwxvkscywgekcnqnkgngokryvfgthpeggwroimsrwwvfmburaaqrzcmtctfxgrlhzbgixfkdjuerowuvtkmawtsrjzucrniwyzeauuizmfdlufwzqotupmykssvwaqljbiegvbonhqaekpeknpvkzgerjljgridlosexgtjkxnnjcqiahuvwvewsjmxbdhsemaoqsfsgxwghtskfxxxtohbqkascghktyxjeznuhguhvkhkhrgxvfjvktljkvmvkrikybfhjrleqxrwvyevynaprnumwpoazwmbevifrcbcckwmhcqrzgyywxlvxjmckmrfitvkelvoubqmflpmuhlwcjtcokvourxueatywmbsgbtcqzmekogznpbqsahgubnsfzfrvlgqvdsqcjptjkztcwbdciaivtwzavpzxgnbtscpzmffbwzezpijzcxnehhecsdyxabdupyqrfxywcikihubgfwrllhdmampqzewshiumisbuabzuthodmfjczwvwwbxbtadhhqqyqdzrfgzlbtlrtayrflxuehrxmggtbglsijmqiozpaydqxcewzizkdauybxbxhekppdxwnrwuchqotbsvatsxallqoatgdlarcoakioexnkugtpdldgzwnjmrdxtaheygwqmesdtinktgfxwsdibsyqyqdypwpzxucwefnzhfwjcnecbhtpuhcmvnyczwlozxlnsiizeteykfiitgyfqjbsoxvtumabgjhaquojutgxzqniiiisqhixjpcnhvopknagkmbuyqbbhlgaqhrnedonfsinfpywjvncbefkdwiidrmjtfunjhbcyvwnbiffildglmcspbmumhyewizqgfwuvbqawruppyjfkmthxpocpbduzqvecdapdjmiuvzduavdasropcdfmrzdfwkisetbznqubivoqkzvxhiuxowfmrsuevnllosjzzeajuxvvayugkzolddxbhyetzqkrxeyomyjidrculppuxxyuswfzkawloetbximriowohrduwpecmczgurxippfecsdrpybwjhbutwgqqrzckigaqevdazolffbffhpmklobchtddrbaqwfddkizyyhpowbgjiswezvhljhcjmetduxyawxjlzthhuplzwidbcbckqxmnitrniupkrqmzxdyctxgaagizoxgpjcedyuewibyalsvlshpngreerfzrqfloevdearlhcdwmxyjoqozxljgbgnccdefycdrqseaqsybwkvzeeccvkpmvpumlhjhsbaenzpbswpcgnxcvkdwllgwubibjaoiadzndfcodoaczxlugycuhdnkcvxjfzptwutlocwctpdfzfkygjrqxavotfrdlmazcfyjsrffzpnzfmuywpjkgywelrhiejhzeuagmlnefnmpscwcpjlcwbkgqwmohylgekxctqmitgtialbugksfirpemtpeixcypilupvlyppqalshwfeeqsqvjorfmpixdzlbiuxmshemapxjrmxhzkrquumpoiqzdgqldzfqohqyfhjeucjsuvjqtlyopxvgauncwthkcuxrtnsobcpkohuygbgltmgogainwmgebuiesejbwgajeaxhbegysmgfhiqdbpgbzsirzjntnfcvjlrqrstbxgznqzzzlxmkwbpvocrzeyquurrjnoblqeohyqfjhmsakicpazwvbvoouiytydwerlxzffwljqtqcwzytmdbpfloyfoixpqivuatqunngyazxvxdnyxhuahilvwnnmhimgxyvtopgsspwzubozebxefsmvsccyiykpoldfmmrnaywhbkcpgjvhuoclwxkxlbszuasmhynflhldmkkecgxjliseppoqrieqwtjkqidksvxctztcpjgedhsgnarqwyliuwbioarpurqdgubcowdepthbqxrtaxzatwzzoxbyuvinokysodoumpztnvcwtsbicuvtvqxszfdpcfdofzkcyqtojcywcsbijvsgitjbdpmqowvvdtllgdbdotqmkgklpizyltljyfdvmluinspznkutjlhwsfudbwiyrmisaqgjlhcneqnoeqrmsztwdkhjqorbfxpkzkihjpgdggwbajszidcvufnjyqxwwxglawjvneegoxztrcyjqlduczzhgdlesnaeyialxfhtcgwkxjcdsllpqwurenryothdqzdbjmppjyvwzxobkvlrxjytmpklararqdqjjnblxaliqhjvtbzysfkbhroccnlwnslpsvkarenxfezocpdocgamvufzcfjkxijwybwgbfmnnwuuunsoupaxbylxggremxxakntirsqjwkyxkldqokrlwevrvoovoekhesvxmbnycclrdhrzzbovalhtnzdhfuyatdgeyazstiovogkiuuvsjvvofvrfwyoxydkgkvhporcxccrlcecgqakknogwyemwcfmokuflsskyevbdkmmumftzcpdonagopprxcmwwuarqxbxglrnprstubwfjmxpwdsribxcglhhzthhajimjawanewsqmwifzndqwojclkdilkisapeegpeixshskpfdnsbmfjiojelllsvuquupkwvnkgfdwreabvhyswnsnsdofccebjqmawlkqbzcrxqcvargeqvruhgypqcfbltnhswzjbjayqglgsyttnvpxrjbbotzcmoscbykzxoqoqkooycfiviewtmpyzzpicglhsydafzdzresxjeqhahsukeprzooumbltzxhmqktoypcjenuqqlkpwtvyscfcxcodnokzxpcjlimqmeltiipawblteiyaftlvefhrglstuwupkfvjzhrlvejljfahcenhnsqmmcfpnbtwrkukzncabvgyvvfqhsairahkulbejckkoapagatvkhceqswlpzijcwddrooijdcircayscwmordpckluyryrguednmhzleeklgggqujqeobgesjdbpuueenraljjecjxssdosskkbhrnykrfvumazfcjalcttxewlxiwtsojrmeakgzkwympgkdrshbiaamlwwwvacewcjgaruzmcpblpgqdyykxjyybhwwgowlcsliiitgffqdfprvrrf";
        let ans = "zzzlxmkwbpvocrzeyquurrjnoblqeohyqfjhmsakicpazwvbvoouiytydwerlxzffwljqtqcwzytmdbpfloyfoixpqivuatqunngyazxvxdnyxhuahilvwnnmhimgxyvtopgsspwzubozebxefsmvsccyiykpoldfmmrnaywhbkcpgjvhuoclwxkxlbszuasmhynflhldmkkecgxjliseppoqrieqwtjkqidksvxctztcpjgedhsgnarqwyliuwbioarpurqdgubcowdepthbqxrtaxzatwzzoxbyuvinokysodoumpztnvcwtsbicuvtvqxszfdpcfdofzkcyqtojcywcsbijvsgitjbdpmqowvvdtllgdbdotqmkgklpizyltljyfdvmluinspznkutjlhwsfudbwiyrmisaqgjlhcneqnoeqrmsztwdkhjqorbfxpkzkihjpgdggwbajszidcvufnjyqxwwxglawjvneegoxztrcyjqlduczzhgdlesnaeyialxfhtcgwkxjcdsllpqwurenryothdqzdbjmppjyvwzxobkvlrxjytmpklararqdqjjnblxaliqhjvtbzysfkbhroccnlwnslpsvkarenxfezocpdocgamvufzcfjkxijwybwgbfmnnwuuunsoupaxbylxggremxxakntirsqjwkyxkldqokrlwevrvoovoekhesvxmbnycclrdhrzzbovalhtnzdhfuyatdgeyazstiovogkiuuvsjvvofvrfwyoxydkgkvhporcxccrlcecgqakknogwyemwcfmokuflsskyevbdkmmumftzcpdonagopprxcmwwuarqxbxglrnprstubwfjmxpwdsribxcglhhzthhajimjawanewsqmwifzndqwojclkdilkisapeegpeixshskpfdnsbmfjiojelllsvuquupkwvnkgfdwreabvhyswnsnsdofccebjqmawlkqbzcrxqcvargeqvruhgypqcfbltnhswzjbjayqglgsyttnvpxrjbbotzcmoscbykzxoqoqkooycfiviewtmpyzzpicglhsydafzdzresxjeqhahsukeprzooumbltzxhmqktoypcjenuqqlkpwtvyscfcxcodnokzxpcjlimqmeltiipawblteiyaftlvefhrglstuwupkfvjzhrlvejljfahcenhnsqmmcfpnbtwrkukzncabvgyvvfqhsairahkulbejckkoapagatvkhceqswlpzijcwddrooijdcircayscwmordpckluyryrguednmhzleeklgggqujqeobgesjdbpuueenraljjecjxssdosskkbhrnykrfvumazfcjalcttxewlxiwtsojrmeakgzkwympgkdrshbiaamlwwwvacewcjgaruzmcpblpgqdyykxjyybhwwgowlcsliiitgffqdfprvrrf";
        assert_eq!(Solution::last_substring(s.to_string()), ans);
    }

    #[test]
    fn test4() {
        let s = "cbcbcbcacbcbcbcb";
        let ans = "cbcbcbcb";
        assert_eq!(Solution::last_substring(s.to_string()), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

impl Solution {
    pub fn last_substring(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut max_char = s[0];
        let mut pos = vec![0];
        for i in 1..s.len() {
            if s[i] > max_char {
                max_char = s[i];
                pos = vec![i];
            } else if s[i] == max_char {
                pos.push(i);
            }
        }
        let mut p = pos[pos.len() - 1];
        for i in (0..pos.len() - 1).rev() {
            let mut winner = pos[i];
            for j in pos[i] + 1..p {
                if p + (j - pos[i]) >= s.len() || s[j] > s[p + (j - pos[i])] {
                    break;
                } else if s[j] < s[p + (j - pos[i])] {
                    winner = p;
                    break;
                }
            }
            p = winner
        }
        s[p..].into_iter().collect()
    }
}
