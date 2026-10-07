#[doc = "Register `FCG0PC` reader"]
pub type R = crate::R<Fcg0pcSpec>;
#[doc = "Register `FCG0PC` writer"]
pub type W = crate::W<Fcg0pcSpec>;
#[doc = "Field `PRT0` reader - desc PRT0"]
pub type Prt0R = crate::BitReader;
#[doc = "Field `PRT0` writer - desc PRT0"]
pub type Prt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCG0PCWE` writer - desc FCG0PCWE"]
pub type Fcg0pcweW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - desc PRT0"]
    #[inline(always)]
    pub fn prt0(&self) -> Prt0R {
        Prt0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PRT0"]
    #[inline(always)]
    pub fn prt0(&mut self) -> Prt0W<'_, Fcg0pcSpec> {
        Prt0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - desc FCG0PCWE"]
    #[inline(always)]
    pub fn fcg0pcwe(&mut self) -> Fcg0pcweW<'_, Fcg0pcSpec> {
        Fcg0pcweW::new(self, 16)
    }
}
#[doc = "desc FCG0PC\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg0pc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg0pc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg0pcSpec;
impl crate::RegisterSpec for Fcg0pcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg0pc::R`](R) reader structure"]
impl crate::Readable for Fcg0pcSpec {}
#[doc = "`write(|w| ..)` method takes [`fcg0pc::W`](W) writer structure"]
impl crate::Writable for Fcg0pcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG0PC to value 0"]
impl crate::Resettable for Fcg0pcSpec {}
