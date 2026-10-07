#[doc = "Register `STR` reader"]
pub type R = crate::R<StrSpec>;
#[doc = "Register `STR` writer"]
pub type W = crate::W<StrSpec>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, StrSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "desc STR\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for StrSpec {}
#[doc = "`write(|w| ..)` method takes [`str::W`](W) writer structure"]
impl crate::Writable for StrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for StrSpec {}
