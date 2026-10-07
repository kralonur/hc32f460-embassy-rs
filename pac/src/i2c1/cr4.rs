#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<Cr4Spec>;
#[doc = "Field `BUSWAIT` reader - desc BUSWAIT"]
pub type BuswaitR = crate::BitReader;
#[doc = "Field `BUSWAIT` writer - desc BUSWAIT"]
pub type BuswaitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - desc BUSWAIT"]
    #[inline(always)]
    pub fn buswait(&self) -> BuswaitR {
        BuswaitR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - desc BUSWAIT"]
    #[inline(always)]
    pub fn buswait(&mut self) -> BuswaitW<'_, Cr4Spec> {
        BuswaitW::new(self, 10)
    }
}
#[doc = "desc CR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for Cr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR4 to value 0x0030_0307"]
impl crate::Resettable for Cr4Spec {
    const RESET_VALUE: u32 = 0x0030_0307;
}
