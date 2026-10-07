#[doc = "Register `DNSEQCTL2` reader"]
pub type R = crate::R<Dnseqctl2Spec>;
#[doc = "Register `DNSEQCTL2` writer"]
pub type W = crate::W<Dnseqctl2Spec>;
#[doc = "Field `DOFFSET` reader - desc DOFFSET"]
pub type DoffsetR = crate::FieldReader<u32>;
#[doc = "Field `DOFFSET` writer - desc DOFFSET"]
pub type DoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DNSCNT` reader - desc DNSCNT"]
pub type DnscntR = crate::FieldReader<u16>;
#[doc = "Field `DNSCNT` writer - desc DNSCNT"]
pub type DnscntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - desc DOFFSET"]
    #[inline(always)]
    pub fn doffset(&self) -> DoffsetR {
        DoffsetR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc DNSCNT"]
    #[inline(always)]
    pub fn dnscnt(&self) -> DnscntR {
        DnscntR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - desc DOFFSET"]
    #[inline(always)]
    pub fn doffset(&mut self) -> DoffsetW<'_, Dnseqctl2Spec> {
        DoffsetW::new(self, 0)
    }
    #[doc = "Bits 20:31 - desc DNSCNT"]
    #[inline(always)]
    pub fn dnscnt(&mut self) -> DnscntW<'_, Dnseqctl2Spec> {
        DnscntW::new(self, 20)
    }
}
#[doc = "desc DNSEQCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dnseqctl2Spec;
impl crate::RegisterSpec for Dnseqctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dnseqctl2::R`](R) reader structure"]
impl crate::Readable for Dnseqctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`dnseqctl2::W`](W) writer structure"]
impl crate::Writable for Dnseqctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DNSEQCTL2 to value 0"]
impl crate::Resettable for Dnseqctl2Spec {}
